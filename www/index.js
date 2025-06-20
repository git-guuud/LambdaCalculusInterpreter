import init, {generate_tree, beta_reduce_once, get_token_rep} from "/pkg/LambdaCalculusInterpreter.js";


async function beta_reduce() {
    await init();
    draw_graph(beta_reduce_once());
    token_rep();
}

function token_rep() {
    var output = document.getElementById("output");
    output.value += get_token_rep() + "\n";
}

async function parse() {
    await init();
    var code = document.getElementById("input").value;
    var output = document.getElementById("output");
    output.value = "";
    draw_graph(generate_tree(code));
    token_rep();
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
        root.setThemes([
            am5themes_Animated.new(root)
        ]);


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
        series.appear(1000, 100);

        return () => {root.dispose();}
    }); // end am5.ready()
}


document.getElementById("Parse").addEventListener("click", parse);
document.getElementById("beta-reduce").addEventListener("click", beta_reduce);