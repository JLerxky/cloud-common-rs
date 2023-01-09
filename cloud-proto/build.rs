// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tonic-build")]
    {
        println!("cargo:rerun-if-changed=protos");
        tonic_build::configure()
            .build_client(true)
            .build_server(true)
            .compile(
                &[
                    "controller.proto",
                    "blockchain.proto",
                    "common.proto",
                    "consensus.proto",
                    "executor.proto",
                    "network.proto",
                    "storage.proto",
                    "crypto.proto",
                    "vm/evm.proto",
                    "health_check.proto",
                    "status_code.proto",
                ],
                &["protos/protos"],
            )?;
    }
    Ok(())
}
