import {{ config.name }}
import codeql.{{ config.name }}.controlflow.ControlFlowGraph
import TestUtils

class MyRelevantNode extends CfgNode {
  MyRelevantNode() { toBeTested(this.getScope()) }
}

import codeql.{{ config.name }}.controlflow.internal.ControlFlowGraphImpl::TestOutput<MyRelevantNode>
