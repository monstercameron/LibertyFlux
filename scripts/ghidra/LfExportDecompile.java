// Decompiles a given list of function addresses to one .c text file each.
// Input file: one hex address per line ("00401000" or "0x00401000");
// blank lines and lines starting with '#' are skipped.
// Output: <addr>.c files plus a manifest.json describing each attempt.
// Output is private analysis material and must stay under .artifacts.
// Args: <inputAddressFile> <outputDir> [timeoutSecsPerFunction]
// Usage (Ghidra headless, from the repo root with JAVA_HOME=tools\jdk):
//   analyzeHeadless <projDir> <project> -process <program> -noanalysis
//     -scriptPath scripts\ghidra
//     -postScript LfExportDecompile.java <inputAddressFile> <outputDir> [timeoutSecs]
//@category Functions

import java.io.File;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import com.google.gson.stream.JsonWriter;

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileOptions;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.decompiler.DecompiledFunction;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSpace;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;

public class LfExportDecompile extends GhidraScript {

	private static final int DEFAULT_TIMEOUT_SECS = 30;

	@Override
	public void run() throws Exception {
		String[] args = getScriptArgs();
		if (args.length < 2) {
			printerr("Usage: LfExportDecompile.java <inputAddressFile> <outputDir> [timeoutSecs]");
			return;
		}
		File inputFile = new File(args[0]);
		File outputDir = new File(args[1]);
		int timeoutSecs = DEFAULT_TIMEOUT_SECS;
		if (args.length >= 3) {
			try {
				timeoutSecs = Integer.parseInt(args[2]);
			}
			catch (NumberFormatException e) {
				printerr("LfExportDecompile: bad timeout '" + args[2] + "', using " +
					DEFAULT_TIMEOUT_SECS);
				timeoutSecs = DEFAULT_TIMEOUT_SECS;
			}
		}
		outputDir.mkdirs();

		List<String> lines = Files.readAllLines(inputFile.toPath(), StandardCharsets.UTF_8);
		List<String> requested = new ArrayList<>();
		for (String line : lines) {
			String s = line.trim();
			if (s.isEmpty() || s.startsWith("#")) {
				continue;
			}
			requested.add(s);
		}

		DecompInterface decompiler = new DecompInterface();
		decompiler.setOptions(new DecompileOptions());
		decompiler.toggleCCode(true);
		decompiler.toggleSyntaxTree(false);
		if (!decompiler.openProgram(currentProgram)) {
			printerr("LfExportDecompile: cannot initialize decompiler: " +
				decompiler.getLastMessage());
			decompiler.dispose();
			return;
		}

		Gson gson = new Gson();
		File manifestFile = new File(outputDir, "manifest.json");
		int ok = 0;
		int failed = 0;
		try {
			JsonWriter writer = new JsonWriter(new OutputStreamWriter(
				new FileOutputStream(manifestFile), StandardCharsets.UTF_8));
			try {
				writer.beginArray();

				FunctionManager funcMgr = currentProgram.getFunctionManager();
				for (String req : requested) {
					monitor.checkCancelled();
					monitor.setMessage("LfExportDecompile: " + req);

					Address addr = parseAddr(req);
					JsonObject entry = new JsonObject();
					entry.addProperty("requested", req);
					if (addr == null) {
						entry.addProperty("status", "bad_address");
						gson.toJson(entry, writer);
						failed++;
						continue;
					}
					entry.addProperty("address", addr.toString());

					Function f = funcMgr.getFunctionAt(addr);
					if (f == null) {
						f = funcMgr.getFunctionContaining(addr);
					}
					if (f == null) {
						entry.addProperty("status", "no_function");
						gson.toJson(entry, writer);
						failed++;
						continue;
					}
					entry.addProperty("name", f.getName());

					String fileName = sanitize(addr.toString()) + ".c";
					DecompileResults results = null;
					try {
						results = decompiler.decompileFunction(f, timeoutSecs, monitor);
					}
					catch (Exception e) {
						entry.addProperty("status", "error");
						entry.addProperty("error", String.valueOf(e.getMessage()));
						gson.toJson(entry, writer);
						failed++;
						continue;
					}
					if (results == null || !results.decompileCompleted()) {
						entry.addProperty("status", "decompile_failed");
						String err = results != null ? results.getErrorMessage() : "null result";
						entry.addProperty("error", err);
						gson.toJson(entry, writer);
						failed++;
						continue;
					}
					DecompiledFunction df = results.getDecompiledFunction();
					if (df == null || df.getC() == null) {
						entry.addProperty("status", "no_output");
						gson.toJson(entry, writer);
						failed++;
						continue;
					}

					File cFile = new File(outputDir, fileName);
					OutputStreamWriter cWriter = new OutputStreamWriter(
						new FileOutputStream(cFile), StandardCharsets.UTF_8);
					try {
						cWriter.write("// " + f.getName() + " @ " + addr.toString() + "\n");
						cWriter.write(df.getC());
						if (!df.getC().endsWith("\n")) {
							cWriter.write("\n");
						}
					}
					finally {
						cWriter.close();
					}
					entry.addProperty("status", "ok");
					entry.addProperty("file", fileName);
					gson.toJson(entry, writer);
					ok++;
				}

				writer.endArray();
			}
			finally {
				writer.close();
			}
		}
		finally {
			decompiler.dispose();
		}

		println("LfExportDecompile: requested=" + requested.size() + " ok=" + ok + " failed=" +
			failed + " output=" + outputDir.getAbsolutePath());
	}

	private Address parseAddr(String s) {
		try {
			s = s.trim();
			if (s.startsWith("0x") || s.startsWith("0X")) {
				s = s.substring(2);
			}
			AddressSpace space =
				currentProgram.getAddressFactory().getDefaultAddressSpace();
			return space.getAddress(s);
		}
		catch (Exception e) {
			return null;
		}
	}

	private String sanitize(String s) {
		return s.replaceAll("[^A-Za-z0-9]", "_");
	}
}
