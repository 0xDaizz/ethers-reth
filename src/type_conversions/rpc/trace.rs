use crate::type_conversions::{ToEthers, ToReth};

use ethers::types::{
    AccountDiff as EthersAccountDiff, Action as EthersAction, ActionType as EthersActionType,
    BlockTrace as EthersBlockTrace, Call as EthersCall, CallResult as EthersCallResult,
    CallType as EthersCallType, ChangedType as EthersChangedType, Create as EthersCreate,
    CreateResult as EthersCreateResult, Diff as EthersDiff, MemoryDiff as EthersMemoryDiff,
    Res as EthersRes, Reward as EthersReward, RewardType as EthersRewardType,
    StateDiff as EthersStateDiff, StorageDiff as EthersStorageDiff, Suicide as EthersSuicide,
    Trace as EthersTrace, TraceType as EthersTraceType, TransactionTrace as EthersTransactionTrace,
    VMExecutedOperation as EthersVMExecutedOperation, VMOperation as EthersVMOperation,
    VMTrace as EthersVMTrace,
};
use reth_revm::primitives::bitvec::macros::internal::funty::Fundamental;
use reth_rpc_types::trace::parity::{
    AccountDiff, Action, CallAction, CallOutput, CallType, ChangedType, CreateAction, CreateOutput,
    Delta, LocalizedTransactionTrace, MemoryDelta, RewardAction, RewardType, SelfdestructAction,
    StateDiff, StorageDelta, TraceOutput, TraceResults, TraceResultsWithTransactionHash, TraceType,
    TransactionTrace, VmExecutedOperation, VmInstruction, VmTrace,
};

use reth_rpc_types::trace::geth::{
    GethDebugTracerConfig, GethDebugTracingCallOptions, GethDebugTracingOptions,
    GethDefaultTracingOptions, GethTrace,
};

use ethers::types::{
    CallFrame, DefaultFrame, FourByteFrame,
    GethDebugTracingCallOptions as EthersDebugTracingCallOptions,
    GethDebugTracingOptions as EthersDebugTracingOptions, GethTrace as EthersGethTrace,
    GethTraceFrame as EthersGethTraceFrame, NoopFrame, PreStateFrame, PreStateMode,
};

/// GethDebugTracingCallOptions (ethers) -> (reth)
impl ToReth<GethDebugTracingCallOptions> for EthersDebugTracingCallOptions {
    fn into_reth(self) -> GethDebugTracingCallOptions {
        GethDebugTracingCallOptions {
            tracing_options: GethDebugTracingOptions::default(),
            state_overrides: None,
            block_overrides: None,
        }
    }
}

/// GethDebugTracingOptions (ethers) -> (reth)
impl ToReth<GethDebugTracingOptions> for EthersDebugTracingOptions {
    fn into_reth(self) -> GethDebugTracingOptions {
        GethDebugTracingOptions {
            config: GethDefaultTracingOptions::default(),
            tracer: None,
            tracer_config: GethDebugTracerConfig::default(),
            timeout: None,
        }
    }
}

/// GethTrace (reth) -> (ethers)
impl ToEthers<EthersGethTrace> for GethTrace {
    fn into_ethers(self) -> EthersGethTrace {
        match self {
            GethTrace::Default(_) => {
                EthersGethTrace::Known(EthersGethTraceFrame::Default(DefaultFrame::default()))
            }
            GethTrace::CallTracer(_) => {
                EthersGethTrace::Known(EthersGethTraceFrame::CallTracer(CallFrame::default()))
            }
            GethTrace::FourByteTracer(_) => EthersGethTrace::Known(
                EthersGethTraceFrame::FourByteTracer(FourByteFrame::default()),
            ),
            GethTrace::PreStateTracer(_) => {
                EthersGethTrace::Known(EthersGethTraceFrame::PreStateTracer(
                    PreStateFrame::Default(PreStateMode::default()),
                ))
            }
            GethTrace::NoopTracer(_) => {
                EthersGethTrace::Known(EthersGethTraceFrame::NoopTracer(NoopFrame::default()))
            }
            GethTrace::JS(value) => EthersGethTrace::Unknown(value),
        }
    }
}

/// Action (ethers) -> (reth)
impl ToReth<Action> for EthersAction {
    fn into_reth(self) -> Action {
        match self {
            EthersAction::Call(a) => Action::Call(CallAction {
                from: a.from.into_reth(),
                to: a.to.into_reth(),
                value: a.value.into_reth(),
                gas: a.gas.into_reth(),
                input: a.input.into_reth(),
                call_type: a.call_type.into_reth(),
            }),
            EthersAction::Create(a) => Action::Create(CreateAction {
                from: a.from.into_reth(),
                value: a.value.into_reth(),
                gas: a.gas.into_reth(),
                init: a.init.into_reth(),
            }),
            EthersAction::Suicide(a) => Action::Selfdestruct(SelfdestructAction {
                address: a.address.into_reth(),
                refund_address: a.refund_address.into_reth(),
                balance: a.balance.into_reth(),
            }),
            EthersAction::Reward(a) => Action::Reward(RewardAction {
                author: a.author.into_reth(),
                value: a.value.into_reth(),
                reward_type: match a.reward_type {
                    EthersRewardType::Uncle => RewardType::Uncle,
                    _ => RewardType::Block,
                },
            }),
        }
    }
}

/// Action (reth) -> (ethers)
impl ToEthers<EthersAction> for Action {
    fn into_ethers(self) -> EthersAction {
        match self {
            Action::Call(a) => EthersAction::Call(EthersCall {
                from: a.from.into_ethers(),
                to: a.to.into_ethers(),
                value: a.value.into_ethers(),
                gas: a.gas.into_ethers(),
                input: a.input.into_ethers(),
                call_type: a.call_type.into_ethers(),
            }),
            Action::Create(a) => EthersAction::Create(EthersCreate {
                from: a.from.into_ethers(),
                value: a.value.into_ethers(),
                gas: a.gas.into_ethers(),
                init: a.init.into_ethers(),
            }),
            Action::Selfdestruct(a) => EthersAction::Suicide(EthersSuicide {
                address: a.address.into_ethers(),
                refund_address: a.refund_address.into_ethers(),
                balance: a.balance.into_ethers(),
            }),
            Action::Reward(a) => EthersAction::Reward(EthersReward {
                author: a.author.into_ethers(),
                value: a.value.into_ethers(),
                reward_type: match a.reward_type {
                    RewardType::Block => EthersRewardType::Block,
                    RewardType::Uncle => EthersRewardType::Uncle,
                },
            }),
        }
    }
}

// -----------------------------------------------

/// CallType (ethers) + (reth)
impl ToReth<CallType> for EthersCallType {
    fn into_reth(self) -> CallType {
        match self {
            EthersCallType::None => CallType::None,
            EthersCallType::Call => CallType::Call,
            EthersCallType::CallCode => CallType::CallCode,
            EthersCallType::DelegateCall => CallType::DelegateCall,
            EthersCallType::StaticCall => CallType::StaticCall,
        }
    }
}

/// CallType (reth) -> (ethers)
impl ToEthers<EthersCallType> for CallType {
    fn into_ethers(self) -> EthersCallType {
        match self {
            CallType::None => EthersCallType::None,
            CallType::Call => EthersCallType::Call,
            CallType::CallCode => EthersCallType::CallCode,
            CallType::DelegateCall => EthersCallType::DelegateCall,
            CallType::StaticCall => EthersCallType::StaticCall,
        }
    }
}

// -----------------------------------------------

/// EthersTrace (ethers) -> TransactionTrace (reth)
impl ToReth<TransactionTrace> for EthersTrace {
    fn into_reth(self) -> TransactionTrace {
        TransactionTrace {
            trace_address: self.trace_address,
            subtraces: self.subtraces,
            action: self.action.into_reth(),
            result: match self.result.into_reth() {
                Some(Some(result)) => Some(result),
                _ => None,
            },
            error: self.error,
        }
    }
}

/// EthersTrace (ethers) + LocalizedTransactionTrace (reth)
impl ToReth<LocalizedTransactionTrace> for EthersTrace {
    fn into_reth(self) -> LocalizedTransactionTrace {
        LocalizedTransactionTrace {
            trace: self.clone().into_reth(),
            transaction_position: self.transaction_position.map(|x| x.as_u64()),
            transaction_hash: self.transaction_hash.into_reth(),
            block_number: Some(self.block_number),
            block_hash: Some(self.block_hash.into_reth()),
        }
    }
}

/// LocalizedTransactionTrace (reth) -> EthersTrace (ethers)
impl ToEthers<EthersTrace> for LocalizedTransactionTrace {
    fn into_ethers(self) -> EthersTrace {
        let action = self.trace.action.into_ethers();
        EthersTrace {
            action: action.clone(),
            result: self.trace.result.clone().into_ethers(),
            trace_address: self.trace.trace_address,
            subtraces: self.trace.subtraces,
            transaction_position: self.transaction_position.map(|x| x as usize),
            transaction_hash: self.transaction_hash.into_ethers(),
            block_number: self.block_number.unwrap(),
            block_hash: self.block_hash.into_ethers().unwrap(),
            action_type: match action {
                EthersAction::Call(_) => EthersActionType::Call,
                EthersAction::Create(_) => EthersActionType::Create,
                EthersAction::Suicide(_) => EthersActionType::Suicide,
                EthersAction::Reward(_) => EthersActionType::Reward,
            },
            error: self.trace.error,
        }
    }
}

// -----------------------------------------------

impl ToReth<CallOutput> for EthersCallResult {
    fn into_reth(self) -> CallOutput {
        CallOutput { gas_used: self.gas_used.into_reth(), output: self.output.into_reth() }
    }
}

impl ToEthers<EthersCallResult> for CallOutput {
    fn into_ethers(self) -> EthersCallResult {
        EthersCallResult {
            gas_used: self.gas_used.into_ethers(),
            output: self.output.into_ethers(),
        }
    }
}

// -----------------------------------------------

impl ToReth<CreateOutput> for EthersCreateResult {
    fn into_reth(self) -> CreateOutput {
        CreateOutput {
            gas_used: self.gas_used.into_reth(),
            code: self.code.into_reth(),
            address: self.address.into_reth(),
        }
    }
}

impl ToEthers<EthersCreateResult> for CreateOutput {
    fn into_ethers(self) -> EthersCreateResult {
        EthersCreateResult {
            gas_used: self.gas_used.into_ethers(),
            code: self.code.into_ethers(),
            address: self.address.into_ethers(),
        }
    }
}

// -----------------------------------------------

impl ToReth<Option<TraceOutput>> for EthersRes {
    fn into_reth(self) -> Option<TraceOutput> {
        match self {
            EthersRes::Call(result) => Some(TraceOutput::Call(result.into_reth())),
            EthersRes::Create(result) => Some(TraceOutput::Create(result.into_reth())),
            EthersRes::None => None,
        }
    }
}

impl ToEthers<EthersRes> for TraceOutput {
    fn into_ethers(self) -> EthersRes {
        match self {
            TraceOutput::Call(result) => EthersRes::Call(result.into_ethers()),
            TraceOutput::Create(result) => EthersRes::Create(result.into_ethers()),
        }
    }
}

// -----------------------------------------------

/// TraceType (ethers) -> (reth)
impl ToReth<TraceType> for EthersTraceType {
    fn into_reth(self) -> TraceType {
        match self {
            EthersTraceType::Trace => TraceType::Trace,
            EthersTraceType::VmTrace => TraceType::VmTrace,
            EthersTraceType::StateDiff => TraceType::StateDiff,
        }
    }
}

/// TraceType (reth) -> (ethers)
impl ToEthers<EthersTraceType> for TraceType {
    fn into_ethers(self) -> EthersTraceType {
        match self {
            TraceType::Trace => EthersTraceType::Trace,
            TraceType::VmTrace => EthersTraceType::VmTrace,
            TraceType::StateDiff => EthersTraceType::StateDiff,
        }
    }
}

// -----------------------------------------------

/// EthersBlockTrace (ethers) -> TraceResults (reth)
impl ToReth<TraceResults> for EthersBlockTrace {
    fn into_reth(self) -> TraceResults {
        TraceResults {
            output: self.output.into_reth(),
            // Ethers represents missing traces as Options, Reth uses empty vectors
            trace: self.trace.into_reth().unwrap_or_default(),
            vm_trace: self.vm_trace.into_reth(),
            state_diff: self.state_diff.into_reth(),
        }
    }
}

/// TraceResults (reth) -> EthersBlockTrace (ethers)
impl ToEthers<EthersBlockTrace> for TraceResults {
    fn into_ethers(self) -> EthersBlockTrace {
        EthersBlockTrace {
            output: self.output.into_ethers(),
            trace: match self.trace.len() {
                0 => None,
                _ => Some(self.trace.into_ethers()),
            },
            vm_trace: self.vm_trace.into_ethers(),
            state_diff: self.state_diff.into_ethers(),
            transaction_hash: None,
        }
    }
}

// -----------------------------------------------

/// EthersBlockTrace (ethers) -> TraceResultsWithTransactionHash (reth)
impl ToReth<TraceResultsWithTransactionHash> for EthersBlockTrace {
    fn into_reth(self) -> TraceResultsWithTransactionHash {
        TraceResultsWithTransactionHash {
            full_trace: self.clone().into_reth(),
            transaction_hash: self.transaction_hash.into_reth().unwrap(),
        }
    }
}

/// TraceResultsWithTransactionHash (reth) -> EthersBlockTrace (ethers)
impl ToEthers<EthersBlockTrace> for TraceResultsWithTransactionHash {
    fn into_ethers(self) -> EthersBlockTrace {
        EthersBlockTrace {
            output: self.full_trace.output.into_ethers(),
            trace: match self.full_trace.trace.len() {
                0 => None,
                _ => Some(self.full_trace.trace.into_ethers()),
            },
            vm_trace: self.full_trace.vm_trace.into_ethers(),
            state_diff: self.full_trace.state_diff.into_ethers(),
            transaction_hash: Some(self.transaction_hash.into_ethers()),
        }
    }
}

// -----------------------------------------------

/// TransactionTrace (ethers) -> (reth)
impl ToReth<TransactionTrace> for EthersTransactionTrace {
    fn into_reth(self) -> TransactionTrace {
        TransactionTrace {
            trace_address: self.trace_address,
            subtraces: self.subtraces,
            action: self.action.into_reth(),
            result: match self.result.into_reth() {
                Some(Some(result)) => Some(result),
                _ => None,
            },
            error: self.error,
        }
    }
}

/// TransactionTrace (reth) -> (ethers)
impl ToEthers<EthersTransactionTrace> for TransactionTrace {
    fn into_ethers(self) -> EthersTransactionTrace {
        EthersTransactionTrace {
            trace_address: self.trace_address,
            subtraces: self.subtraces,
            action: self.action.clone().into_ethers(),
            result: self.result.clone().into_ethers(),
            action_type: match self.action.into_ethers() {
                EthersAction::Call(_) => EthersActionType::Call,
                EthersAction::Create(_) => EthersActionType::Create,
                EthersAction::Suicide(_) => EthersActionType::Suicide,
                EthersAction::Reward(_) => EthersActionType::Reward,
            },
            error: self.error,
        }
    }
}

// -----------------------------------------------

/// VMTrace (ethers) -> (reth)
impl ToReth<VmTrace> for EthersVMTrace {
    fn into_reth(self) -> VmTrace {
        VmTrace { code: self.code.into_reth(), ops: self.ops.into_reth() }
    }
}

/// VMTrace (reth) -> (ethers)
impl ToEthers<EthersVMTrace> for VmTrace {
    fn into_ethers(self) -> EthersVMTrace {
        EthersVMTrace { code: self.code.into_ethers(), ops: self.ops.into_ethers() }
    }
}

// -----------------------------------------------

/// EthersVMOperation (ethers) -> VmInstruction (reth)
impl ToReth<VmInstruction> for EthersVMOperation {
    fn into_reth(self) -> VmInstruction {
        VmInstruction {
            pc: self.pc,
            cost: self.cost,
            ex: self.ex.into_reth(),
            sub: self.sub.into_reth(),
        }
    }
}

/// VmInstruction (reth) -> EthersVMOperation (ethers)
impl ToEthers<EthersVMOperation> for VmInstruction {
    fn into_ethers(self) -> EthersVMOperation {
        EthersVMOperation {
            pc: self.pc,
            cost: self.cost,
            ex: self.ex.into_ethers(),
            sub: self.sub.into_ethers(),
            op: Default::default(),
        }
    }
}

// -----------------------------------------------

/// VmExecutedOperation (ethers) -> (reth)
impl ToReth<VmExecutedOperation> for EthersVMExecutedOperation {
    fn into_reth(self) -> VmExecutedOperation {
        VmExecutedOperation {
            used: self.used,
            push: Some(self.push[0]).into_reth(), // Check this
            mem: self.mem.into_reth(),
            store: self.store.into_reth(),
        }
    }
}

/// VmExecutedOperation (reth) -> (ethers)
impl ToEthers<EthersVMExecutedOperation> for VmExecutedOperation {
    fn into_ethers(self) -> EthersVMExecutedOperation {
        EthersVMExecutedOperation {
            used: self.used,
            push: vec![self.push.into_ethers().unwrap()], // Check this
            mem: self.mem.into_ethers(),
            store: self.store.into_ethers(),
        }
    }
}

// -----------------------------------------------

/// EthersMemoryDiff (ethers) -> MemoryDelta (reth)
impl ToReth<MemoryDelta> for EthersMemoryDiff {
    fn into_reth(self) -> MemoryDelta {
        MemoryDelta { off: self.off, data: self.data.into_reth() }
    }
}

/// MemoryDelta (reth) -> EthersMemoryDiff (ethers)
impl ToEthers<EthersMemoryDiff> for MemoryDelta {
    fn into_ethers(self) -> EthersMemoryDiff {
        EthersMemoryDiff { off: self.off, data: self.data.into_ethers() }
    }
}

// -----------------------------------------------

/// EthersStorageDiff (ethers) -> StorageDelta '(reth)
impl ToReth<StorageDelta> for EthersStorageDiff {
    fn into_reth(self) -> StorageDelta {
        StorageDelta { key: self.key.into_reth(), val: self.val.into_reth() }
    }
}

/// StorageDelta (reth) -> EthersStorageDiff (ethers)
impl ToEthers<EthersStorageDiff> for StorageDelta {
    fn into_ethers(self) -> EthersStorageDiff {
        EthersStorageDiff { key: self.key.into_ethers(), val: self.val.into_ethers() }
    }
}

// -----------------------------------------------

/// AccountDiff (ethers) -> (reth)
impl ToReth<AccountDiff> for EthersAccountDiff {
    fn into_reth(self) -> AccountDiff {
        AccountDiff {
            balance: self.balance.into_reth(),
            nonce: self.nonce.into_reth(),
            code: self.code.into_reth(),
            storage: self.storage.into_reth(),
        }
    }
}

/// AccountDiff (reth) -> (ethers)
impl ToEthers<EthersAccountDiff> for AccountDiff {
    fn into_ethers(self) -> EthersAccountDiff {
        EthersAccountDiff {
            balance: self.balance.into_ethers(),
            nonce: self.nonce.into_ethers(),
            code: self.code.into_ethers(),
            storage: self.storage.into_ethers(),
        }
    }
}
// -----------------------------------------------

/// StateDiff (ethers) -> (reth)
impl ToReth<StateDiff> for EthersStateDiff {
    fn into_reth(self) -> StateDiff {
        StateDiff(self.0.into_reth())
    }
}

/// StateDiff (reth) -> (ethers)
impl ToEthers<EthersStateDiff> for StateDiff {
    fn into_ethers(self) -> EthersStateDiff {
        EthersStateDiff(self.0.into_ethers())
    }
}

// -----------------------------------------------

/// Diff (ethers) -> Delta (reth)
impl<T, F> ToReth<Delta<T>> for EthersDiff<F>
where
    F: ToReth<T>,
    T: Clone,
{
    fn into_reth(self) -> Delta<T> {
        match self {
            EthersDiff::Same => Delta::Unchanged,
            EthersDiff::Born(x) => Delta::Added(x.into_reth()),
            EthersDiff::Died(x) => Delta::Removed(x.into_reth()),
            EthersDiff::Changed(x) => Delta::Changed(x.into_reth()),
        }
    }
}

/// Delta (reth) -> Diff (ethers)
impl<F, T> ToEthers<EthersDiff<F>> for Delta<T>
where
    T: ToEthers<F>,
    F: Clone,
{
    fn into_ethers(self) -> EthersDiff<F> {
        match self {
            Delta::Unchanged => EthersDiff::Same,
            Delta::Added(x) => EthersDiff::Born(x.into_ethers()),
            Delta::Removed(x) => EthersDiff::Died(x.into_ethers()),
            Delta::Changed(x) => EthersDiff::Changed(x.into_ethers()),
        }
    }
}

// -----------------------------------------------

/// ChangedType (ethers) -> (reth)
impl<T, F> ToReth<ChangedType<T>> for EthersChangedType<F>
where
    F: ToReth<T>,
    T: Clone,
{
    fn into_reth(self) -> ChangedType<T> {
        ChangedType { from: self.from.into_reth(), to: self.to.into_reth() }
    }
}

/// ChangedType (reth) -> (ethers)
impl<F, T> ToEthers<EthersChangedType<F>> for ChangedType<T>
where
    T: ToEthers<F>,
    F: Clone,
{
    fn into_ethers(self) -> EthersChangedType<F> {
        EthersChangedType { from: self.from.into_ethers(), to: self.to.into_ethers() }
    }
}
