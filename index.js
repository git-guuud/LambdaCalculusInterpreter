import init, {generate_tree} from "/pkg/LambdaCalculusInterpreter.js";

async function parse() {
    await init();
    var code = document.getElementById("input").value;

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

        var data = JSON.parse(generate_tree(code));

        series.data.setAll([data]);
        series.set("selectedDataItem", series.dataItems[0]);
        series.appear(1000, 100);

    }); // end am5.ready()

}


document.getElementById("Parse").addEventListener("click", parse);