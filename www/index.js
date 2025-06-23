import initSync, {generate_tree, beta_reduce_once, get_token_rep} from "./LambdaCalculusInterpreter.js";

var parsed = false;
async function beta_reduce() {
    await initSync();
    if (!parsed) {
        alert("Please parse the input first.");
        return;
    }
    draw_graph(beta_reduce_once());
    token_rep();
}

async function solve() 
{
    await initSync();
    if (!parsed) {
        alert("Please parse the input first.");
        return;
    }
    var exp = get_token_rep();
    var t = "";
    do
    {
        exp = get_token_rep();
        beta_reduce_once();
        token_rep();
    } while (exp != get_token_rep());
    draw_graph(beta_reduce_once());
}

function token_rep() {
    var output = document.getElementById("output");
    output.value += get_token_rep() + "\n";
}

async function parse() {
    await initSync();
    parsed = true;
    var code = document.getElementById("input").value;
    var output = document.getElementById("output");
    output.value = "";
    try 
    {
        var data = generate_tree(code);
    }
    catch (e) 
    {
        alert("Unknown error: " + e.message);
        return;
    }
    if (data == "Error parsing input") {
        alert("Error while parsing, please check your input.");
    }
    else {
        draw_graph(data);
        token_rep();
    }
}

function draw_graph(tree_data) 
{
    am5.array.each(am5.registry.rootElements,
        function (root) {
            if (root.dom.id == "chartdiv") {
                root.dispose();
            }
        }
    );

    am5.ready(function () {


        // Create root element
        // https://www.amcharts.com/docs/v5/getting-started/#Root_element
        var root = am5.Root.new("chartdiv");


        // Set themes
        // https://www.amcharts.com/docs/v5/concepts/themes/
        // root.setThemes([
        //     am5themes_Animated.new(root)
        // ]);


        var zoomableContainer = root.container.children.push(
            am5.ZoomableContainer.new(root, {
                width: am5.p100,
                height: am5.p100,
                wheelable: true,
                pinchZoom: true
            })
        );

        var zoomTools = zoomableContainer.children.push(am5.ZoomTools.new(root, {
            target: zoomableContainer
        }));

        // Create series
        // https://www.amcharts.com/docs/v5/charts/hierarchy/#Adding
        var series = zoomableContainer.contents.children.push(am5hierarchy.Tree.new(root, {
            singleBranchOnly: false,
            downDepth: 1,
            initialDepth: 10,
            categoryField: "name",
            childDataField: "children"
        }));

        series.labels.template.set("minScale", 0);

        var data = JSON.parse(tree_data);

        series.data.setAll([data]);
        series.set("selectedDataItem", series.dataItems[0]);
        // series.get("colors").setAll({colors:[am5.color(0x508484)]});
        series.appear(1000, 100);

        return () => {root.dispose();}
    }); // end am5.ready()
}


document.getElementById("Parse").addEventListener("click", parse);
document.getElementById("beta-reduce").addEventListener("click", beta_reduce);
document.getElementById("solve").addEventListener("click", solve);


document.getElementById("help-button").addEventListener("click", function() {
    document.getElementById("help-overlay").style.display = "initial";
    document.getElementById("help-overlay").style.opacity = 1;
});
document.getElementById("close-help").addEventListener("click", function() {
    document.getElementById("help-overlay").style.opacity = 0;
    document.getElementById("help-overlay").style.display = "none";
});