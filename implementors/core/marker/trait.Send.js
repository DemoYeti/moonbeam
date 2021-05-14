(function() {var implementors = {};
implementors["account"] = [{"text":"impl Send for EthereumSignature","synthetic":true,"types":[]},{"text":"impl Send for EthereumSigner","synthetic":true,"types":[]}];
implementors["moonbeam"] = [{"text":"impl Send for Extensions","synthetic":true,"types":[]},{"text":"impl Send for ExtensionsFork","synthetic":true,"types":[]},{"text":"impl Send for Executor","synthetic":true,"types":[]},{"text":"impl Send for Subcommand","synthetic":true,"types":[]},{"text":"impl Send for BuildSpecCommand","synthetic":true,"types":[]},{"text":"impl Send for ExportGenesisStateCommand","synthetic":true,"types":[]},{"text":"impl Send for ExportGenesisWasmCommand","synthetic":true,"types":[]},{"text":"impl Send for RunCmd","synthetic":true,"types":[]},{"text":"impl Send for Cli","synthetic":true,"types":[]},{"text":"impl Send for RelayChainCli","synthetic":true,"types":[]},{"text":"impl Send for Sealing","synthetic":true,"types":[]},{"text":"impl Send for EthApi","synthetic":true,"types":[]},{"text":"impl Send for MockValidationDataInherentDataProvider","synthetic":true,"types":[]},{"text":"impl&lt;C, P, BE&gt; Send for FullDeps&lt;C, P, BE&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_extensions_evm"] = [{"text":"impl&lt;'config, S&gt; Send for TraceExecutorWrapper&lt;'config, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_core_debug"] = [{"text":"impl Send for TraceParams","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_core_trace"] = [{"text":"impl Send for FilterRequest","synthetic":true,"types":[]},{"text":"impl Send for RequestBlockId","synthetic":true,"types":[]},{"text":"impl Send for RequestBlockTag","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_core_txpool"] = [{"text":"impl Send for Transaction","synthetic":true,"types":[]},{"text":"impl Send for Summary","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for TxPoolResult&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_debug"] = [{"text":"impl Send for Debug","synthetic":true,"types":[]},{"text":"impl&lt;B, C, BE&gt; Send for DebugHandler&lt;B, C, BE&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_primitives_debug"] = [{"text":"impl Send for TransactionTrace","synthetic":true,"types":[]},{"text":"impl Send for TransactionTraceAction","synthetic":true,"types":[]},{"text":"impl Send for TransactionTraceOutput","synthetic":true,"types":[]},{"text":"impl Send for TransactionTraceResult","synthetic":true,"types":[]},{"text":"impl Send for TraceType","synthetic":true,"types":[]},{"text":"impl Send for TransactionTrace","synthetic":true,"types":[]},{"text":"impl Send for RawStepLog","synthetic":true,"types":[]},{"text":"impl Send for CallInner","synthetic":true,"types":[]},{"text":"impl Send for Call","synthetic":true,"types":[]},{"text":"impl Send for CallResult","synthetic":true,"types":[]},{"text":"impl Send for CreateResult","synthetic":true,"types":[]},{"text":"impl Send for CallType","synthetic":true,"types":[]},{"text":"impl Send for CreateType","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_trace"] = [{"text":"impl&lt;B, C&gt; Send for Trace&lt;B, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for CacheBatchId","synthetic":true,"types":[]},{"text":"impl Send for CacheRequester","synthetic":true,"types":[]},{"text":"impl&lt;B, C, BE&gt; Send for CacheTask&lt;B, C, BE&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_txpool"] = [{"text":"impl&lt;B, C, P&gt; Send for TxPool&lt;B, C, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_runtime"] = [{"text":"impl Send for SessionKeys","synthetic":true,"types":[]},{"text":"impl Send for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Send for Version","synthetic":true,"types":[]},{"text":"impl Send for BlockWeights","synthetic":true,"types":[]},{"text":"impl Send for BlockLength","synthetic":true,"types":[]},{"text":"impl Send for SS58Prefix","synthetic":true,"types":[]},{"text":"impl Send for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Send for MaxLocks","synthetic":true,"types":[]},{"text":"impl Send for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl Send for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Send for MoonbeamGasWeightMapping","synthetic":true,"types":[]},{"text":"impl Send for BlockGasLimit","synthetic":true,"types":[]},{"text":"impl Send for MaximumSchedulerWeight","synthetic":true,"types":[]},{"text":"impl Send for CouncilMotionDuration","synthetic":true,"types":[]},{"text":"impl Send for CouncilMaxProposals","synthetic":true,"types":[]},{"text":"impl Send for CouncilMaxMembers","synthetic":true,"types":[]},{"text":"impl Send for TechComitteeMotionDuration","synthetic":true,"types":[]},{"text":"impl Send for TechComitteeMaxProposals","synthetic":true,"types":[]},{"text":"impl Send for TechComitteeMaxMembers","synthetic":true,"types":[]},{"text":"impl Send for LaunchPeriod","synthetic":true,"types":[]},{"text":"impl Send for VotingPeriod","synthetic":true,"types":[]},{"text":"impl Send for FastTrackVotingPeriod","synthetic":true,"types":[]},{"text":"impl Send for EnactmentPeriod","synthetic":true,"types":[]},{"text":"impl Send for CooloffPeriod","synthetic":true,"types":[]},{"text":"impl Send for MinimumDeposit","synthetic":true,"types":[]},{"text":"impl Send for MaxVotes","synthetic":true,"types":[]},{"text":"impl Send for MaxProposals","synthetic":true,"types":[]},{"text":"impl Send for PreimageByteDeposit","synthetic":true,"types":[]},{"text":"impl Send for InstantAllowed","synthetic":true,"types":[]},{"text":"impl Send for TransactionConverter","synthetic":true,"types":[]},{"text":"impl Send for ReservedXcmpWeight","synthetic":true,"types":[]},{"text":"impl Send for MinBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Send for DefaultBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Send for BondDuration","synthetic":true,"types":[]},{"text":"impl Send for MinSelectedCandidates","synthetic":true,"types":[]},{"text":"impl Send for MaxNominatorsPerCollator","synthetic":true,"types":[]},{"text":"impl Send for MaxCollatorsPerNominator","synthetic":true,"types":[]},{"text":"impl Send for DefaultCollatorCommission","synthetic":true,"types":[]},{"text":"impl Send for MinCollatorStk","synthetic":true,"types":[]},{"text":"impl Send for MinNominatorStk","synthetic":true,"types":[]},{"text":"impl Send for Runtime","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl !Send for Origin","synthetic":true,"types":[]},{"text":"impl Send for OriginCaller","synthetic":true,"types":[]},{"text":"impl Send for PalletInfo","synthetic":true,"types":[]},{"text":"impl Send for Call","synthetic":true,"types":[]},{"text":"impl Send for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Send for RuntimeApi","synthetic":true,"types":[]},{"text":"impl&lt;Block:&nbsp;BlockT, C:&nbsp;CallApiAt&lt;Block&gt;&gt; Send for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C::StateBackend: StateBackend&lt;HashFor&lt;Block&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["pallet_author_mapping"] = [{"text":"impl&lt;T&gt; Send for Pallet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for GenesisConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AuthorId: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, Inner&gt; Send for MappedEventHandler&lt;T, Inner&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Inner: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, Inner&gt; Send for MappedCanAuthor&lt;T, Inner&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Inner: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, Inner&gt; Send for MappedFindAuthor&lt;T, Inner&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Inner: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["pallet_ethereum_chain_id"] = [{"text":"impl&lt;T&gt; Send for Pallet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for GenesisConfig","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for _GeneratedPrefixForStorageChainId&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["parachain_staking"] = [{"text":"impl&lt;T&gt; Send for Range&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Balance&gt; Send for InflationInfo&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for SubstrateWeight&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Pallet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Send for Bond&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for CollatorStatus","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Send for CollatorSnapshot&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Send for Collator&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Send for Nominator&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;BlockNumber&gt; Send for RoundInfo&lt;BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Event&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for GenesisConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for _GeneratedPrefixForStorageAtStake&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for _GeneratedPrefixForStorageStaked&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for _GeneratedPrefixForStorageInflationConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for _GeneratedPrefixForStoragePoints&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for _GeneratedPrefixForStorageAwardedPts&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["precompiles"] = [{"text":"impl&lt;R&gt; Send for MoonbeamPrecompiles&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Send,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()