// Exports the call graph of the current program to a JSON array file.
// Each element: {"caller": <entry address>, "callee": <entry address>}.
// Args: <outputJsonPath>
// Usage (Ghidra headless, from the repo root with JAVA_HOME=tools\jdk):
//   analyzeHeadless <projDir> <project> -process <program> -noanalysis
//     -scriptPath scripts\ghidra
//     -postScript LfExportCallGraph.java <outputJsonPath>
//@category Functions

import java.io.File;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;
import java.nio.charset.StandardCharsets;
import java.util.Set;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import com.google.gson.stream.JsonWriter;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;

public class LfExportCallGraph extends GhidraScript {

	@Override
	public void run() throws Exception {
		String[] args = getScriptArgs();
		if (args.length < 1) {
			printerr("Usage: LfExportCallGraph.java <outputJsonPath>");
			return;
		}
		File outFile = new File(args[0]);
		if (outFile.getParentFile() != null) {
			outFile.getParentFile().mkdirs();
		}

		Gson gson = new Gson();
		int functions = 0;
		int edges = 0;

		JsonWriter writer = new JsonWriter(
			new OutputStreamWriter(new FileOutputStream(outFile), StandardCharsets.UTF_8));
		try {
			writer.beginArray();

			FunctionIterator iter = currentProgram.getFunctionManager().getFunctions(true);
			while (iter.hasNext()) {
				monitor.checkCancelled();
				Function f = iter.next();
				functions++;
				String caller = f.getEntryPoint().toString();
				Set<Function> called;
				try {
					called = f.getCalledFunctions(monitor);
				}
				catch (Exception e) {
					continue;
				}
				for (Function callee : called) {
					JsonObject obj = new JsonObject();
					obj.addProperty("caller", caller);
					obj.addProperty("callee", callee.getEntryPoint().toString());
					gson.toJson(obj, writer);
					edges++;
				}

				if (functions % 2000 == 0) {
					monitor.setMessage("LfExportCallGraph: " + functions + " functions...");
				}
			}

			writer.endArray();
		}
		finally {
			writer.close();
		}

		println("LfExportCallGraph: functions=" + functions + " edges=" + edges +
			" output=" + outFile.getAbsolutePath());
	}
}
