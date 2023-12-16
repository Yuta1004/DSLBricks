
        // Note: This is auto generated file.

        import Blockly from "blockly";

        
            Blockly.Blocks["brick"] = {
                init: 
            function() {
                this.jsonInit({
                    type: "brick",
                    message0: "DSLBrick %1 %2",
                    args0: [
                {
                    type: "field_variable",
                    name: "DSLBrick",
                },
                {
                    type: "input_dummy"
                }
                ],
                    colour: 200,
                    tooltop: "",
                    helpUrl: "",
                    previousStatement: "null",nextStatement: "null"
                })
            }
        
            }
        

            Blockly.Blocks["std.primitive.Integer"] = {
                init: 
            function() {
                this.jsonInit({
                    type: "std.primitive.Integer",
                    message0: "Integer %1 variable %2 %3 property %4 %5",
                    args0: [
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "field_variable",
                    name: "variable",
                },
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "field_input",
                    name: "property",
                    text: ""
                },
                {
                    type: "input_dummy"
                }
                ],
                    colour: 200,
                    tooltop: "",
                    helpUrl: "",
                    
                })
            }
        
            }
        

            Blockly.Blocks["std.primitive.Float"] = {
                init: 
            function() {
                this.jsonInit({
                    type: "std.primitive.Float",
                    message0: "Float %1 variable %2 %3 property %4 %5",
                    args0: [
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "field_variable",
                    name: "variable",
                },
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "field_input",
                    name: "property",
                    text: ""
                },
                {
                    type: "input_dummy"
                }
                ],
                    colour: 200,
                    tooltop: "",
                    helpUrl: "",
                    
                })
            }
        
            }
        

            Blockly.Blocks["std.statement.c.If"] = {
                init: 
            function() {
                this.jsonInit({
                    type: "std.statement.c.If",
                    message0: "If %1 variable %2 %3 property %4 %5 cond %6 %7 stmt %8 %9",
                    args0: [
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "field_variable",
                    name: "variable",
                },
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "field_input",
                    name: "property",
                    text: ""
                },
                {
                    type: "input_dummy"
                }
                ,
                {
                    type: "input_dummy"
                },
                {
                    type: "input_statement",
                    name: "cond"
                }
                ,
                {
                    type: "input_dummy"
                },
                {
                    type: "input_statement",
                    name: "stmt"
                }
                ],
                    colour: 200,
                    tooltop: "",
                    helpUrl: "",
                    
                })
            }
        
            }
        

    
