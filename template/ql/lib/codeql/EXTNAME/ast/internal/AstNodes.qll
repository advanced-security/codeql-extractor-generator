private import TreeSitter

/**
 * An AST node of a {{ config.display_name }} program
 * */
cached
newtype TAstNode =
  {%- for astnode in ts.all %}
  T{{ astnode }}({{ config.name | upcase }}::{{ astnode }} r) {% if forloop.length != forloop.last -%} or{%- endif %}
  {%- endfor %}


{%- if ts.literals.size != 0  %}
/**
 * A literal value in a {{ config.display_name }} program
 */
class TLiterals = {%- for literal in ts.literals %}
  T{{- literal }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}


{%- if ts.statements.size != 0  %}
/**
 * A statement in a {{ config.display_name }} program
 */
class TStmt = {%- if ts.loops.size != 0  %}TLoops or {%- endif %}
{% for statement in ts.statements %}
  T{{- statement }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}


{%- if ts.expressions.size != 0  %}
/**
 * A expersion value in a {{ config.display_name }} program
 */
class TExpr = 
{%- if ts.block.size != 0  %}TBlock or {%- endif %}
{% if ts.literals.size != 0  %}TLiterals or {%- endif %}
{% if ts.conditionals.size != 0  %}TConditionalExpr or {%- endif %}
{%- for expr in ts.expressions %}
  T{{- expr }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{%- if ts.conditionals.size != 0  %}
/**
 * A expersion value in a {{ config.display_name }} program
 */
class TConditionalExpr = {%- for cond in ts.conditionals %}
  T{{- cond }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}


{%- if ts.loops.size != 0  %}
/**
 * A loops in a {{ config.display_name }} program
 */
class TLoops = {% for loop in ts.loops %}
  T{{- loop }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

cached
{{ config.name | upcase }}::AstNode toTreeSitter(TAstNode n) {
  {%- for astnode in ts.all %}
  n = T{{ astnode }}(result) {% if forloop.length != forloop.last -%} or{%- endif %}
  {%- endfor %}
}
