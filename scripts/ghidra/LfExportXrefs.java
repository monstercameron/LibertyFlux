// Exports data references from inside functions to a JSON array file.
// Each element: from_function (entry of the containing function), from address,
// to address, ref_type, to_symbol (primary symbol at target, if any) and
// to_value (data representation at target, truncated, if any).
// Only data references are recorded (calls are covered by LfExportCallGraph).
// Args: <outputJsonPath>
// Usage (Ghidra headless, from the repo root with JAVA_HOME=tools\jdk):
//   analyzeHeadless <projDir> <project> -process <program> -noanalysis
//     -scriptPath scripts\ghidra
//     -postScript LfExportXrefs.java <outputJsonPath>
//@category Functions

import java.io.File;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;
import java.nio.charset.StandardCharsets;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonObject;
import com.google.gson.stream.JsonWriter;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.InstructionIterator;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.ReferenceManager;
import ghidra.program.model.symbol.Symbol;

public class LfExportXrefs extends GhidraScript {

	private static final int MAX_VALUE_CHARS = 300;

	@Override
	public void run() throws Exception {
		String[] args = getScriptArgs();
		if (args.length < 1) {
			printerr("Usage: LfExportXrefs.java <outputJsonPath>");
			return;
		}
		File outFile = new File(args[0]);
		if (outFile.getParentFile() != null) {
			outFile.getParentFile().mkdirs();
		}

		Gson gson = new GsonBuilder().serializeNulls().create();
		int functions = 0;
		int refs = 0;

		Listing listing = currentProgram.getListing();
		ReferenceManager refMgr = currentProgram.getReferenceManager();

		JsonWriter writer = new JsonWriter(
			new OutputStreamWriter(new FileOutputStream(outFile), StandardCharsets.UTF_8));
		try {
			writer.beginArray();

			FunctionIterator iter = currentProgram.getFunctionManager().getFunctions(true);
			while (iter.hasNext()) {
				monitor.checkCancelled();
				Function f = iter.next();
				functions++;
				String fromFunction = f.getEntryPoint().toString();

				AddressSetView body;
				try {
					body = f.getBody();
				}
				catch (Exception e) {
					continue;
				}
				InstructionIterator instructions = listing.getInstructions(body, true);
				while (instructions.hasNext()) {
					Instruction insn = instructions.next();
					Address from = insn.getMinAddress();
					Reference[] references;
					try {
						references = refMgr.getReferencesFrom(from);
					}
					catch (Exception e) {
						continue;
					}
					for (Reference ref : references) {
						if (!ref.getReferenceType().isData()) {
							continue;
						}
						Address to = ref.getToAddress();
						if (to == null || !to.isMemoryAddress()) {
							continue;
						}

						String toSymbol = null;
						try {
							Symbol sym = currentProgram.getSymbolTable().getPrimarySymbol(to);
							if (sym != null) {
								toSymbol = sym.getName();
							}
						}
						catch (Exception e) {
							// leave null
						}

						String toValue = null;
						try {
							Data data = listing.getDataAt(to);
							if (data != null) {
								toValue = data.getDefaultValueRepresentation();
								if (toValue != null && toValue.length() > MAX_VALUE_CHARS) {
									toValue = toValue.substring(0, MAX_VALUE_CHARS);
								}
							}
						}
						catch (Exception e) {
							// leave null
						}

						JsonObject obj = new JsonObject();
						obj.addProperty("from_function", fromFunction);
						obj.addProperty("from", from.toString());
						obj.addProperty("to", to.toString());
						obj.addProperty("ref_type", ref.getReferenceType().getName());
						obj.addProperty("to_symbol", toSymbol);
						obj.addProperty("to_value", toValue);
						gson.toJson(obj, writer);
						refs++;
					}
				}

				if (functions % 1000 == 0) {
					monitor.setMessage("LfExportXrefs: " + functions + " functions...");
				}
			}

			writer.endArray();
		}
		finally {
			writer.close();
		}

		println("LfExportXrefs: functions=" + functions + " data_refs=" + refs +
			" output=" + outFile.getAbsolutePath());
	}
}
