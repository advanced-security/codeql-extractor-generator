import codeql.{{ config.name }}.dataflow.internal.TaintTrackingPublic as Public

module Private {
  import codeql.{{ config.name }}.dataflow.DataFlow::DataFlow as DataFlow
  import codeql.{{ config.name }}.dataflow.internal.DataFlowImpl as DataFlowInternal
  import codeql.{{ config.name }}.dataflow.internal.TaintTrackingPrivate
}