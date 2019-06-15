// Copyleft 2018-2019 Superstring.Community
// This file is part of Susy.

// Susy is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Susy is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Susy.  If not, see <http://www.gnu.org/licenses/>.

import java.util.Vector;
import java.util.concurrent.atomic.AtomicInteger;
import io.susy.sophon.Susy;

class Main {
	public static final int ONE_MINUTE_AS_MILLIS = 60 * 1000;

	public static final String[] rpc_queries = {
		"{\"method\":\"susy_versionInfo\",\"params\":[],\"id\":1,\"jsonrpc\":\"2.0\"}",
		"{\"method\":\"sof_getTransactionReceipt\",\"params\":[\"0x444172bef57ad978655171a8af2cfd89baa02a97fcb773067aef7794d6913fff\"],\"id\":1,\"jsonrpc\":\"2.0\"}",
		"{\"method\":\"sof_estimateGas\",\"params\":[{\"from\":\"0x0066Dc48bb833d2B59f730F33952B3c29fE926F5\"}],\"id\":1,\"jsonrpc\":\"2.0\"}",
		"{\"method\":\"sof_getBalance\",\"params\":[\"0x0066Dc48bb833d2B59f730F33952B3c29fE926F5\"],\"id\":1,\"jsonrpc\":\"2.0\"}"
	};

	public static final String[] ws_queries = {
		"{\"method\":\"susy_subscribe\",\"params\":[\"sof_getBalance\",[\"0xcd2a3d9f938e13cd947ec05abc7fe734df8dd826\",\"latest\"]],\"id\":1,\"jsonrpc\":\"2.0\"}",
		"{\"method\":\"susy_subscribe\",\"params\":[\"susy_netPeers\"],\"id\":1,\"jsonrpc\":\"2.0\"}",
		"{\"method\":\"sof_subscribe\",\"params\":[\"newHeads\"],\"id\":1,\"jsonrpc\":\"2.0\"}"
	};

	public static void runSusy(String[] config) {
		String loggerMode = "rpc=trace";
		String loggerFile = "foo.log";
		Susy susy = new Susy(config, loggerMode, loggerFile);

		Callback rpcCallback = new Callback(1);
		Callback webSocketCallback = new Callback(2);

		for (String query : rpc_queries) {
			susy.rpcQuery(query, ONE_MINUTE_AS_MILLIS, rpcCallback);
		}

		while (rpcCallback.getNumCallbacks() != 4);

		Vector<Long> sessions = new Vector<Long>();

		for (String ws : ws_queries) {
			long session = susy.subscribeWebSocket(ws, webSocketCallback);
			sessions.add(session);
		}

		try {
			Thread.sleep(ONE_MINUTE_AS_MILLIS);
		} catch (Exception e) {
			System.out.println(e);
		}

		for (long session : sessions) {
			susy.unsubscribeWebSocket(session);
		}

		// Force GC to destroy susy
		susy = null;
		System.gc();
	}

	public static void main(String[] args) {
		String[] full = {"--no-ipc" , "--susy-jsonrpc-apis=all", "--chain", "kovan"};
		String[] light = {"--no-ipc", "--light", "--susy-jsonrpc-apis=all", "--chain", "kovan"};

		runSusy(full);

		try {
			Thread.sleep(ONE_MINUTE_AS_MILLIS);
		} catch (Exception e) {
			System.out.println(e);
		}

		runSusy(light);
	}
}

class Callback {
	private AtomicInteger counter;
	private final int callbackType;

	public Callback(int type) {
		counter = new AtomicInteger();
		callbackType = type;
	}

	public void callback(Object response) {
		counter.getAndIncrement();
	}

	public int getNumCallbacks() {
		return counter.intValue();
	}
}
