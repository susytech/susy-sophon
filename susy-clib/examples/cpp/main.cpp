// Copyleft 2015-2019 Superstring.Community
// This file is part of Susy Sophon.

// Susy Sophon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Susy Sophon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Susy Sophon.  If not, see <http://www.gnu.org/licenses/>.

#include <chrono>
#include <susy.h>
#include <regex>
#include <string>
#include <thread>

void* susy_run(std::vector<const char*>);
int susy_subscribe_to_websocket(void*);
int susy_rpc_queries(void*);

const int SUBSCRIPTION_ID_LEN = 18;
const size_t TIMEOUT_ONE_MIN_AS_MILLIS = 60 * 1000;
const unsigned int CALLBACK_RPC = 1;
const unsigned int CALLBACK_WS = 2;

struct Callback {
	unsigned int type;
	long unsigned int counter;
};

// list of rpc queries
const std::vector<std::string> rpc_queries {
	"{\"method\":\"susy_versionInfo\",\"params\":[],\"id\":1,\"jsonrpc\":\"2.0\"}",
	"{\"method\":\"sof_getTransactionReceipt\",\"params\":[\"0x444172bef57ad978655171a8af2cfd89baa02a97fcb773067aef7794d6913fff\"],\"id\":1,\"jsonrpc\":\"2.0\"}",
	"{\"method\":\"sof_estimateGas\",\"params\":[{\"from\":\"0x0066Dc48bb833d2B59f730F33952B3c29fE926F5\"}],\"id\":1,\"jsonrpc\":\"2.0\"}",
	"{\"method\":\"sof_getBalance\",\"params\":[\"0x0066Dc48bb833d2B59f730F33952B3c29fE926F5\"],\"id\":1,\"jsonrpc\":\"2.0\"}"
};

// list of subscriptions
const std::vector<std::string> ws_subscriptions {
	"{\"method\":\"susy_subscribe\",\"params\":[\"sof_getBalance\",[\"0xcd2a3d9f938e13cd947ec05abc7fe734df8dd826\",\"latest\"]],\"id\":1,\"jsonrpc\":\"2.0\"}",
	"{\"method\":\"susy_subscribe\",\"params\":[\"susy_netPeers\"],\"id\":1,\"jsonrpc\":\"2.0\"}",
	"{\"method\":\"sof_subscribe\",\"params\":[\"newHeads\"],\"id\":1,\"jsonrpc\":\"2.0\"}"
};

// callback that gets invoked upon an event
void callback(void* user_data, const char* response, size_t _len) {
	Callback* cb = static_cast<Callback*>(user_data);
	if (cb->type == CALLBACK_RPC) {
		cb->counter -= 1;
	} else if (cb->type == CALLBACK_WS) {
		std::regex is_subscription ("\\{\"jsonrpc\":\"2.0\",\"result\":\"0[xX][a-fA-F0-9]{16}\",\"id\":1\\}");
		if (std::regex_match(response, is_subscription) == true) {
			cb->counter -= 1;
		}
	}
}

int main() {
	// run full-client
	{
		std::vector<const char*> config = {"--no-ipc" , "--susy-jsonrpc-apis=all", "--chain", "kovan"};
		void* susy = susy_run(config);
		if (susy_rpc_queries(susy)) {
			printf("rpc_queries failed\r\n");
			return 1;
		}

		if (susy_subscribe_to_websocket(susy)) {
			printf("ws_queries failed\r\n");
			return 1;
		}

		if (susy != nullptr) {
			susy_destroy(susy);
		}
	}

	// run light-client
	{
		std::vector<const char*> light_config = {"--no-ipc", "--light", "--susy-jsonrpc-apis=all", "--chain", "kovan"};
		void* susy = susy_run(light_config);

		if (susy_rpc_queries(susy)) {
			printf("rpc_queries failed\r\n");
			return 1;
		}

		if (susy_subscribe_to_websocket(susy)) {
			printf("ws_queries failed\r\n");
			return 1;
		}

		if (susy != nullptr) {
			susy_destroy(susy);
		}
	}
	return 0;
}

int susy_rpc_queries(void* susy) {
	if (!susy) {
		return 1;
	}

	Callback cb { .type = CALLBACK_RPC, .counter = rpc_queries.size() };

	for (auto query : rpc_queries) {
		if (susy_rpc(susy, query.c_str(), query.length(), TIMEOUT_ONE_MIN_AS_MILLIS, callback, &cb) != 0) {
			return 1;
		}
	}

	while(cb.counter != 0);
	return 0;
}


int susy_subscribe_to_websocket(void* susy) {
	if (!susy) {
		return 1;
	}

	std::vector<const void*> sessions;

	Callback cb { .type = CALLBACK_WS, .counter = ws_subscriptions.size() };

	for (auto sub : ws_subscriptions) {
		void *const session = susy_subscribe_ws(susy, sub.c_str(), sub.length(), callback, &cb);
		if (!session) {
			return 1;
		}
		sessions.push_back(session);
	}

	while(cb.counter != 0);
	std::this_thread::sleep_for(std::chrono::seconds(60));
	for (auto session : sessions) {
		susy_unsubscribe_ws(session);
	}
	return 0;
}

void* susy_run(std::vector<const char*> args) {
	SusyParams cfg = {
		.configuration = nullptr,
		.on_client_restart_cb = callback,
		.on_client_restart_cb_custom = nullptr,
		.logger = nullptr
	};

	std::vector<size_t> str_lens;

	for (auto arg: args) {
		str_lens.push_back(std::strlen(arg));
	}

	// make sure no out-of-range access happens here
	if (args.empty()) {
		if (susy_config_from_cli(nullptr, nullptr, 0, &cfg.configuration) != 0) {
			return nullptr;
		}
	} else {
		if (susy_config_from_cli(&args[0], &str_lens[0], args.size(), &cfg.configuration) != 0) {
			return nullptr;
		}
	}

	// enable logging but only the `rpc module` and don't write it to a file
	char log_mode [] = "rpc=trace";
	susy_set_logger(log_mode, strlen(log_mode), nullptr, 0, &cfg.logger);

	void *susy = nullptr;
	if (susy_start(&cfg, &susy) != 0) {
		return nullptr;
	}

	return susy;
}
