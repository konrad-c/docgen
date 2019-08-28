{
    "int": ${<1>int},
    "int": ${<1>int},
    "int_min_max": ${int:-100,100},
    "float": ${float},
    "float_rounded": ${float:3},
    "name::full": "${<1>name::full}",
    "name::full": "${<1>name::full}",
    "name::first": "${<1>name::first}",
    "name::first": "${<1>name::first}",
    "name::last": "${<1>name::last}",
    "name::last": "${<1>name::last}",
    "place": "${place}",
    "address": "${address}",
    "guid": "${guid}",
    "phone": "${<1>phone}",
    "phone::mobile": "${<1>phone::landline}",
    "phone::landline": "${<1>phone::mobile}",
    "set": "${set:First,Second,Third}"
    "set_with_spaces": "${set:first one,then second,finally third}"
}