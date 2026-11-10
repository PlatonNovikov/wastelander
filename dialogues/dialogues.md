### This explanation is pretty poorly written, so it’s best to just use the existing dialogues as examples.
# GHDL - gkfnjy's horrible dialogue language

A dialogue must begin with a `start` node. The text for a node is specified using the `text` parameter. Responses are defined within a sub-node called `<parent_node>.answers`, which includes the following parameters:

* `text` (required) — the text of the response.
* `next` (required if `end` is not defined) — the name of the next node.
* `condition` — the condition under which this response becomes available. It must be specified in the format `<variable> <comparison_operator> <number/bool>` (this can be followed by further logical comparisons with additional conditions). Conditions are evaluated in the order they are listed.
* `set` - assignment of a variable. It must be specified in the format `<variable> <assignment_operator> <number/bool>`.
* `end` — if the dialogue should conclude after this response, this must be set to `true`.

## Variables

There are several types of variables:
* Local variables – variables that exist only within the scope of a specific dialogue. They are identified by the `local.` prefix; they can be signed 16-bit integers or booleans. By default, they are initialized to `0` or `false`.
* Global variables – variables accessible across all dialogues. They are identified by the `global.` prefix; they can be signed 16-bit integers or booleans. By default, they are initialized to `0` or `false`.
* Character stats – immutable character attributes. These can only be used for comparison purposes and consist of a set of predefined names.

## Comparison
The folowing comparison operators are available:
#### Numerical comparison operators:
* \>= (more or equal)
* <= (less or equal)
* == (equal)
* != (not equal)
* \> (more)
* < (less)

#### Logical operators:
* && (and)
<!-- * || (or) -->
<!-- * != (xor) -->

## Stats
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

