// Applies function names from a JSON file.
// Input: JSON array of {"address": <hex, with or without 0x>, "name": <name>}.
// Addresses without a function are skipped (counted, not created).
// Args: <inputJsonPath>
// Usage (Ghidra headless, from the repo root with JAVA_HOME=tools\jdk):
//   analyzeHeadless <projDir> <project> -process <program> -noanalysis
//     -scriptPath scripts\ghidra
//     -postScript LfApplySymbols.java <inputJsonPath>
//@category Functions

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSpace;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.symbol.SourceType;

public class LfApplySymbols extends GhidraScript {

	@Override
	public void run() throws Exception {
		String[] args = getScriptArgs();
		if (args.length < 1) {
			printerr("Usage: LfApplySymbols.java <inputJsonPath>");
			return;
		}
		File inputFile = new File(args[0]);
		String text = Files.readString(inputFile.toPath(), StandardCharsets.UTF_8);
		JsonElement root = JsonParser.parseString(text);
		if (!root.isJsonArray()) {
			printerr("LfApplySymbols: input is not a JSON array: " + inputFile);
			return;
		}
		JsonArray items = root.getAsJsonArray();

		FunctionManager funcMgr = currentProgram.getFunctionManager();
		int applied = 0;
		int skipped = 0;
		int failed = 0;
		int index = 0;
		for (JsonElement element : items) {
			monitor.checkCancelled();
			index++;
			if (!element.isJsonObject()) {
				printerr("LfApplySymbols: item " + index + " is not an object, skipped");
				skipped++;
				continue;
			}
			JsonObject obj = element.getAsJsonObject();
			if (!obj.has("address") || !obj.has("name")) {
				printerr("LfApplySymbols: item " + index + " lacks address/name, skipped");
				skipped++;
				continue;
			}
			String addrText = obj.get("address").getAsString();
			String name = obj.get("name").getAsString();
			Address addr = parseAddr(addrText);
			if (addr == null) {
				printerr("LfApplySymbols: bad address '" + addrText + "', skipped");
				skipped++;
				continue;
			}
			Function f = funcMgr.getFunctionAt(addr);
			if (f == null) {
				printerr("LfApplySymbols: no function at " + addrText + ", skipped");
				skipped++;
				continue;
			}
			try {
				f.setName(name, SourceType.USER_DEFINED);
				applied++;
			}
			catch (Exception e) {
				printerr("LfApplySymbols: cannot rename " + addrText + " to '" + name + "': " +
					e.getMessage());
				failed++;
			}

			if (index % 2000 == 0) {
				monitor.setMessage("LfApplySymbols: " + index + " symbols...");
			}
		}

		println("LfApplySymbols: total=" + items.size() + " applied=" + applied + " skipped=" +
			skipped + " failed=" + failed);
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
}
