// Exports every function in the current program to a JSON array file.
// Each element: entry address, name, size in bytes, instruction count,
// calling convention, thunk flag, external flag, library-namespace flag,
// library flag (external OR library namespace), section (memory block),
// caller count and callee count. Caller/callee counts are -1 when they
// could not be computed. External (imported) functions are included so
// that every callee in LfExportCallGraph output has a matching record.
// Args: <outputJsonPath>
// Usage (Ghidra headless, from the repo root with JAVA_HOME=tools\jdk):
//   analyzeHeadless <projDir> <project> -process <program> -noanalysis
//     -scriptPath scripts\ghidra
//     -postScript LfExportFunctions.java <outputJsonPath>
//@category Functions

import java.io.File;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;
import java.nio.charset.StandardCharsets;
import java.util.Iterator;
import java.util.Set;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonObject;
import com.google.gson.stream.JsonWriter;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;
import ghidra.program.model.listing.InstructionIterator;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.Namespace;

public class LfExportFunctions extends GhidraScript {

	private Gson gson;
	private Listing listing;
	private int count;
	private int libraryCount;

	@Override
	public void run() throws Exception {
		String[] args = getScriptArgs();
		if (args.length < 1) {
			printerr("Usage: LfExportFunctions.java <outputJsonPath>");
			return;
		}
		File outFile = new File(args[0]);
		if (outFile.getParentFile() != null) {
			outFile.getParentFile().mkdirs();
		}

		gson = new GsonBuilder().serializeNulls().create();
		listing = currentProgram.getListing();
		count = 0;
		libraryCount = 0;

		JsonWriter writer = new JsonWriter(
			new OutputStreamWriter(new FileOutputStream(outFile), StandardCharsets.UTF_8));
		try {
			writer.beginArray();

			FunctionIterator iter = currentProgram.getFunctionManager().getFunctions(true);
			while (iter.hasNext()) {
				monitor.checkCancelled();
				writeFunction(writer, iter.next());
			}
			Iterator<Function> externals =
				currentProgram.getFunctionManager().getExternalFunctions();
			while (externals.hasNext()) {
				monitor.checkCancelled();
				writeFunction(writer, externals.next());
			}

			writer.endArray();
		}
		finally {
			writer.close();
		}

		println("LfExportFunctions: functions=" + count + " library=" + libraryCount +
			" output=" + outFile.getAbsolutePath());
	}

	private void writeFunction(JsonWriter writer, Function f) throws Exception {
		Address entry = f.getEntryPoint();

		long size = 0;
		int insns = 0;
		try {
			AddressSetView body = f.getBody();
			size = body.getNumAddresses();
			InstructionIterator instructions = listing.getInstructions(body, true);
			while (instructions.hasNext()) {
				instructions.next();
				insns++;
			}
		}
		catch (Exception e) {
			// External or odd functions may have no countable body; leave zeros.
		}

		int callers = -1;
		int callees = -1;
		try {
			Set<Function> calling = f.getCallingFunctions(monitor);
			callers = calling.size();
		}
		catch (Exception e) {
			// leave -1 to signal "could not compute"
		}
		try {
			Set<Function> called = f.getCalledFunctions(monitor);
			callees = called.size();
		}
		catch (Exception e) {
			// leave -1 to signal "could not compute"
		}

		boolean external = false;
		try {
			external = f.isExternal();
		}
		catch (Exception e) {
			// leave false
		}
		boolean libraryNs = isInLibraryNamespace(f);
		boolean library = external || libraryNs;
		if (library) {
			libraryCount++;
		}

		String section = null;
		try {
			MemoryBlock block = currentProgram.getMemory().getBlock(entry);
			if (block != null) {
				section = block.getName();
			}
		}
		catch (Exception e) {
			// leave null
		}

		String cc = null;
		try {
			cc = f.getCallingConventionName();
		}
		catch (Exception e) {
			// leave null
		}

		JsonObject obj = new JsonObject();
		obj.addProperty("entry", entry.toString());
		obj.addProperty("name", f.getName());
		obj.addProperty("size", size);
		obj.addProperty("instructions", insns);
		obj.addProperty("calling_convention", cc);
		obj.addProperty("is_thunk", f.isThunk());
		obj.addProperty("is_external", external);
		obj.addProperty("library_namespace", libraryNs);
		obj.addProperty("is_library", library);
		obj.addProperty("section", section);
		obj.addProperty("callers", callers);
		obj.addProperty("callees", callees);
		gson.toJson(obj, writer);
		count++;

		if (count % 2000 == 0) {
			monitor.setMessage("LfExportFunctions: " + count + " functions...");
		}
	}

	private boolean isInLibraryNamespace(Function f) {
		try {
			Namespace ns = f.getParentNamespace();
			for (int depth = 0; depth < 16 && ns != null; depth++) {
				if (ns.isLibrary()) {
					return true;
				}
				Namespace parent = ns.getParentNamespace();
				if (parent == null || parent.equals(ns)) {
					return false;
				}
				ns = parent;
			}
		}
		catch (Exception e) {
			// fall through
		}
		return false;
	}
}
