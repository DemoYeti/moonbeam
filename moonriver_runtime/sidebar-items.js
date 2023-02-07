window.SIDEBAR_ITEMS = {"constant":[["DAYS",""],["EXTRINSIC_BASE_WEIGHT",""],["FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX","The asset precompile address prefix. Addresses that match against this prefix will be routed to Erc20AssetsPrecompileSet being marked as foreign"],["GAS_PER_SECOND","Current approximation of the gas/s consumption considering EVM execution over compiled WASM (on 4.4Ghz CPU). Given the 500ms Weight, from which 75% only are used for transactions, the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000."],["HOURS",""],["LOCAL_ASSET_PRECOMPILE_ADDRESS_PREFIX","The asset precompile address prefix. Addresses that match against this prefix will be routed to Erc20AssetsPrecompileSet being marked as local"],["MAXIMUM_BLOCK_WEIGHT","Maximum weight per block"],["MILLISECS_PER_BLOCK",""],["MINUTES",""],["VERSION","This runtime version. The spec_version is composed of 2x2 digits. The first 2 digits represent major changes that can’t be skipped, such as data migration upgrades. The last 2 digits represent minor changes which can be skipped."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""],["WEEKS",""],["WEIGHT_PER_GAS","Approximate ratio of the amount of Weight per Gas. u64 works for approximations because Weight is a very small unit compared to gas."]],"enum":[["OriginCaller",""],["PrecompileName",""],["ProxyType","The type used to represent the kinds of proxying allowed."],["RuntimeCall",""],["RuntimeEvent",""]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"macro":[["get",""]],"mod":[["api",""],["asset_config","Asset configuration for Moonbase."],["currency","MOVR, the native token, uses 18 decimals of precision."],["governance","Governance configurations"],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core datastructures."],["xcm_config","XCM configuration for Moonbase."]],"struct":[["AdjustmentVariable","The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to change the fees more rapidly. This low value causes changes to occur slowly over time."],["BabeDataGetter",""],["BlockGasLimit",""],["BlockLength","We allow for 5 MB blocks."],["DealWithFees",""],["FindAuthorAdapter","The author inherent provides a AccountId20, but pallet evm needs an H160. This simple adapter makes the conversion."],["FixedGasPrice",""],["GenesisAccount","Account definition used for genesis block construction."],["GenesisConfig",""],["InflationInfo",""],["InitializationPayment",""],["LengthToFee",""],["MaintenanceDmpHandler",""],["MaintenanceFilter","Maintenance mode Call filter"],["MaintenanceHooks","The hooks we wantt to run in Maintenance Mode"],["MaximumMultiplier","Maximum multiplier. We pick a value that is expensive but not impossibly so; it should act as a safety net."],["MaximumSchedulerWeight",""],["MinimumMultiplier","Minimum amount of the multiplier. This value cannot be too low. A test case should ensure that combined with `AdjustmentVariable`, we can recover from the minimum. See `multiplier_can_grow_from_zero` in integration_tests.rs. This value is currently only used by pallet-transaction-payment as an assertion that the next multiplier is always > min value."],["NormalDmpHandler",""],["NormalFilter","Normal Call Filter We dont allow to create nor mint assets, this for now is disabled We only allow transfers. For now creation of assets will go through asset-manager, while minting/burning only happens through xcm messages This can change in the future"],["OnChargeEVMTransaction",""],["OnNewRound",""],["OrbiterReserveIdentifier",""],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["PayoutCollatorOrOrbiterReward",""],["PrecompilesValue",""],["ProposalBond",""],["Range",""],["RelaySignaturesThreshold",""],["ReservedDmpWeight",""],["ReservedXcmpWeight",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeBlockWeights",""],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SignatureNetworkIdentifier",""],["TargetBlockFullness","The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less than this will decrease the weight and more will increase."],["TransactionConverter",""],["TreasuryId",""],["Version",""],["WeightPerGas",""],["XcmExecutionManager",""]],"trait":[["BuildStorage","Complex storage builder stuff."],["Get","A trait for querying a single value from a type."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them, but you never know…"],["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["AssetId","AssetId type"],["AssetManager",""],["Assets",""],["AuthorFilter",""],["AuthorFilterConfig",""],["AuthorInherent",""],["AuthorMapping",""],["AuthorMappingConfig",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["BlockNumber","An index to a block."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["ConvictionVoting",""],["CouncilCollective",""],["CouncilCollectiveConfig",""],["CrowdloanRewards",""],["CrowdloanRewardsConfig",""],["CumulusXcm",""],["Democracy",""],["DemocracyConfig",""],["DigestItem","Digest item type."],["DmpQueue",""],["EVM",""],["EVMConfig",""],["Ethereum",""],["EthereumChainId",""],["EthereumChainIdConfig",""],["EthereumConfig",""],["Executive","Executive: handles dispatch to the various pallets."],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Identity",""],["Index","Index of a transaction in the chain."],["LocalAssets",""],["MaintenanceMode",""],["MaintenanceModeConfig",""],["Migrations",""],["MigrationsConfig",""],["MoonbeamOrbiters",""],["MoonriverPrecompiles","The PrecompileSet installed in the Moonriver runtime. We include the nine Istanbul precompiles (https://github.com/ethereum/go-ethereum/blob/3c46f557/core/vm/contracts.go#L69) as well as a special precompile for dispatching Substrate extrinsics The following distribution has been decided for the precompiles 0-1023: Ethereum Mainnet Precompiles 1024-2047 Precompiles that are not in Ethereum Mainnet but are neither Moonbeam specific 2048-4095 Moonbeam specific precompiles"],["OpenTechCommitteeCollective",""],["OpenTechCommitteeCollectiveConfig",""],["Origins",""],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainStaking",""],["ParachainStakingConfig",""],["ParachainSystem",""],["PolkadotXcm",""],["PolkadotXcmConfig",""],["Precompiles",""],["Preimage",""],["Proxy",""],["ProxyGenesisCompanion",""],["ProxyGenesisCompanionConfig",""],["Randomness",""],["RandomnessCollectiveFlip",""],["Referenda",""],["Scheduler",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["SlowAdjustingFeeUpdate","Parameterized slow adjusting fee updated based on https://w3f-research.readthedocs.io/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism // editorconfig-checker-disable-line"],["System",""],["SystemConfig",""],["TechCommitteeCollective",""],["TechCommitteeCollectiveConfig",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentConfig",""],["Treasury",""],["TreasuryConfig",""],["TreasuryCouncilCollective",""],["TreasuryCouncilCollectiveConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""],["Whitelist",""],["XTokens",""],["XcmTransactor",""],["XcmpQueue",""]]};