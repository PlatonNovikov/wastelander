### This explanation is pretty poorly written, so it’s best to just use the existing dialogues as examples.
# GHDL - gkfnjy's horrible dialogue language

A dialogue must begin with a `start` node. The text for a node is specified using the `text` parameter. Responses are defined within a sub-node called `<parent_node>.answers`, which includes the following parameters:

* `text` (required) — the text of the response.
* `next` (required if `end` is not defined) — the name of the next node.
* `condition` — the condition under which this response becomes available. Contains a single expression. The response becomes available if the expression is non-zero. The expression is executed after an answer is selected. For example, if the expression is `[condition] = "global.variable += 1"`
then one will be added to `global.variable` only once this answer has been chosen.
* `set` - Contains a list of sequentially executed `expression`. Executed after an answer is selected.
* `end` — if the dialogue should conclude after this response, this must be set to `true`.

## Expressions

Expressions consist of an arbitrary number of operations performed on variables.

### Operations
All operations have the same priority and are performed sequentially. Parentheses will be used to specify the order.  
The following operations exist:

#### Logical
Logical operations use 0 as false and any non-zero value as true. They return 0 if false, or 1 if true
* `||` - or
* `&&` - and

#### Numerical
* `==` - equal
* `!=` - not equal
* `>` -  greater than
* `>=` - greater than or equal to
* `<` - less than
* `<=` - less than or equal to

#### Mathematical
* `+` - addition
* `-` - substraction
* `*` - multiplication
* `/` - division

#### Assignment
* `=`
* `+=`
* `-=`
* `*=`
* `/=`

### Variables

There are several types of variables:
* Local variables – variables that exist only within the scope of a specific dialogue. They are identified by the `local.` prefix; they are signed 16-bit integers. By default, they are initialized to `0`.
* Global variables – variables accessible across all dialogues. They are identified by the `global.` prefix; they are signed 16-bit integers. By default, they are initialized to `0`.
* Character stats – immutable character attributes. These can only be used for comparison purposes and consist of a set of predefined names.

### Stats
The following stats are available:
#### special (with a range from 1 to 10):
* strenght
* perception
* endurance
* charisma
* intelligence
* agility
* luck

#### skills (with a range from 0 to 300):
* barter
* e_weapons (energy weapons)
* explosives
* guns
* lockpick
* medicine
* m_weapons (melee weapons)
* repair
* science
* sneak
* speech
* survival
* unarmed

