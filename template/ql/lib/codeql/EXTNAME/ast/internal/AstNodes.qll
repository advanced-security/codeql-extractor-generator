private import TreeSitter

/**
 * An AST node of a {{ config.display_name }} program
 * */
cached
newtype TAstNode =
  {%- for astnode in ts.all %}
  T{{ astnode }}({{ config.name | upcase }}::{{ astnode }} r) {% if forloop.length != forloop.last -%} or{%- endif %}
  {%- endfor %}


{%- if ts.Literals.size != 0  %}
/**
 * A literal value in a {{ config.display_name }} program
 */
class TLiterals = {%- for literal in ts.Literals %}
  T{{- literal }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{%- if ts.StmtSequence.size != 0  %}
/**
 *  A sequence of statements in a {{ config.display_name }} program
 */
class TStmtSeq = {%- for seq in ts.StmtSequence %}
  T{{- seq }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{%- if ts.Stmts.size != 0  %}
/**
 * A statement in a {{ config.display_name }} program
 */
class TStmts = {%- if ts.Loops.size != 0  %}TLoops or {%- endif %}
{% for statement in ts.Stmts %}
  T{{- statement }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{% if ts.Idents.size != 0  %}
/**
 * An identifier in a {{ config.display_name }} program
 */
class TIdents = {%- for ident in ts.Idents %}
  T{{- ident }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{%- if ts.Expr.size != 0  %}
/**
 * A expersion value in a {{ config.display_name }} program
 */
class TExpr = 
{%- if ts.StmtSequence.size != 0  %}TStmtSeq or {%- endif %}
{% if ts.Idents.size != 0  %}TIdents or {%- endif %}
{% if ts.Calls.size != 0  %}TCalls or {%- endif %}
{% if ts.Literals.size != 0  %}TLiterals or {%- endif %}
{% if ts.Conditionals.size != 0  %}TConditionals or {%- endif %}
{%- for expr in ts.Expr %}
  T{{- expr }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{%- if ts.Conditionals.size != 0  %}
/**
 * A expersion value in a {{ config.display_name }} program
 */
class TConditionals = {%- for cond in ts.Conditionals %}
  T{{- cond }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{% if ts.Calls.size != 0  %}
/**
 * A call expression in a {{ config.display_name }} program
 */
class TCalls = {%- for call in ts.Calls %}
  T{{- call }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

{%- if ts.Loops.size != 0  %}
/**
 * A loops in a {{ config.display_name }} program
 */
class TLoops = {% for loop in ts.Loops %}
  T{{- loop }} {%- if forloop.length != forloop.last %} or {% endif %}
{%- endfor %};
{%- endif %}

cached
{{ config.name | upcase }}::AstNode toTreeSitter(TAstNode n) {
  {%- for astnode in ts.all %}
  n = T{{ astnode }}(result) {% if forloop.length != forloop.last -%} or{%- endif %}
  {%- endfor %}
}
