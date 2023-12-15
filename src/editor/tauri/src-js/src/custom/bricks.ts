import Blockly from "blockly";

Blockly.Blocks["if"] = {
    init: function() {
        this.jsonInit({
            "type": "if",
            "message0": "If (std.statement.c) %1 variable %2 %3 property %4 %5 cond %6 %7 stmt %8 %9",
            "args0": [
                {
                    "type": "input_dummy"
                },
                {
                    "type": "field_variable",
                    "name": "NAME",
                    "variable": "item"
                },
                {
                    "type": "input_dummy"
                },
                {
                    "type": "field_input",
                    "name": "NAME",
                    "text": "Executable"
                },
                {
                    "type": "input_dummy"
                },
                {
                    "type": "input_dummy"
                },
                {
                    "type": "input_statement",
                    "name": "NAME"
                },
                {
                    "type": "input_dummy"
                },
                {
                    "type": "input_statement",
                    "name": "NAME"
                }
            ],
            "colour": 230,
            "tooltip": "",
            "helpUrl": ""
        })
    }
}

Blockly.Blocks["test"] = {
                init: function() {
                    this.jsonInit({
                        "type": "test",
                        "message0": "Test %1 var_0 %2 %3 text_0 %4 %5 blocks_0 %6 %7",
                        "args0": [
                    {
                        "type": "input_dummy"
                    }
                    ,
                    {
                        "type": "field_variable",
                        "name": "NAME",
                        "variable": "var"
                    },
                    {
                        "type": "input_dummy"
                    }
                    ,
                    {
                        "type": "field_input",
                        "name": "NAME",
                        "text": "Executable"
                    },
                    {
                        "type": "input_dummy"
                    }
                    ,
                    {
                        "type": "input_dummy"
                    },
                    {
                        "type": "input_statement",
                        "name": "NAME"
                    }
                    ],
                        "colour": 200,
                        "tooltop": "",
                        "helpUrl": ""
                    })
                }
            }

Blockly.Blocks["brick"] = {
    init: function() {
        this.jsonInit({
            "type": "brick",
            "message0": "DSLBrick %1",
            "args0": [
                {
                    "type": "field_variable",
                    "name": "NAME",
                    "variable": "item"
                }
            ],
            "inputsInline": false,
            "previousStatement": null,
            "nextStatement": null,
            "colour": 230,
            "tooltip": "",
            "helpUrl": ""
        })
    }
}
