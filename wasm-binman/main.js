import init, { process_random, process_force_atlas2, process_circular, process_fruchterman_reingold, process_stress_majorization, process_multidimensional_scaling } from './pkg/graph_layout.js';
import * as d3 from "https://cdn.jsdelivr.net/npm/d3@7/+esm";

let uploadedAlgorithm = null;

function updateControlVisibility(algorithm) {
    const iterationsAlgorithms = ['process_force_atlas2', 'process_fruchterman_reingold', 'process_kamada_kawai', 'process_stress_majorization', 'process_multidimensional_scaling'];
    const gravityAlgorithms = ['process_force_atlas2', 'process_fruchterman_reingold'];
    const scalingRatioAlgorithms = ['process_force_atlas2'];

    document.getElementById('iterations-group').style.display = iterationsAlgorithms.includes(algorithm) ? 'flex' : 'none';
    document.getElementById('gravity-group').style.display = gravityAlgorithms.includes(algorithm) ? 'flex' : 'none';
    document.getElementById('scaling-ratio-group').style.display = scalingRatioAlgorithms.includes(algorithm) ? 'flex' : 'none';
}

async function run(graphStr, algorithm, iterations, gravity, scalingRatio) {
    await init();

    let processedGraph;

    switch (algorithm) {
        case "process_random":
            processedGraph = process_random(graphStr);
            break;
        case "process_force_atlas2":
            processedGraph = process_force_atlas2(graphStr, iterations, gravity, scalingRatio);
            break;
        case "process_circular":
            processedGraph = process_circular(graphStr);
            break;
        case "process_fruchterman_reingold":
            processedGraph = process_fruchterman_reingold(graphStr, iterations, gravity);
            break;
        case "process_stress_majorization":
            processedGraph = process_stress_majorization(graphStr, iterations);
            break;
        case "process_multidimensional_scaling":
            processedGraph = process_multidimensional_scaling(graphStr, iterations);
            break;
        case "uploaded_algorithm":
            if (uploadedAlgorithm) {
                processedGraph = uploadedAlgorithm(graphStr); // Just pass graphStr
            } else {
                throw new Error("No uploaded algorithm available");
            }
            break;
        default:
            throw new Error("Unknown algorithm");
    }

    let { nodes, edges } = parseGraph(processedGraph);

    scaleGraph(nodes);
    updateGraph(nodes, edges);
}

function parseGraph(graphStr) {
    const parts = graphStr.split('edges:');
    const nodesPart = parts[0].split('nodes:')[1]?.trim();
    const edgesPart = parts[1]?.trim();

    if (!nodesPart || !edgesPart) {
        throw new Error("Invalid graph format.");
    }

    const nodes = nodesPart.split(';').filter(Boolean).map((node, index) => {
        const [x, y] = node.split(',').map(Number);
        return { id: index, x, y };
    });

    const edges = edgesPart.split(',').filter(Boolean).map(edge => {
        const [source, target] = edge.split('-').map(Number);
        return { source, target };
    });

    return { nodes, edges };
}

function scaleGraph(nodes) {
    const svg = d3.select("svg");
    const width = +svg.attr("width");
    const height = +svg.attr("height");
    const padding = 20;

    const xExtent = d3.extent(nodes, d => d.x);
    const yExtent = d3.extent(nodes, d => d.y);

    const xScale = d3.scaleLinear()
        .domain([xExtent[0], xExtent[1]])
        .range([padding, width - padding]);

    const yScale = d3.scaleLinear()
        .domain([yExtent[0], yExtent[1]])
        .range([padding, height - padding]);

    nodes.forEach(node => {
        node.x = xScale(node.x);
        node.y = yScale(node.y);
    });
}

function updateGraph(nodes, edges) {
    const svg = d3.select("svg");
    const width = +svg.attr("width");
    const height = +svg.attr("height");

    svg.selectAll("*").remove();

    // Draw grid
    const gridSize = 50;
    svg.append("g")
        .attr("class", "grid")
        .selectAll("line")
        .data(d3.range(0, Math.max(width, height), gridSize))
        .enter().append("line")
        .attr("x1", d => d)
        .attr("y1", 0)
        .attr("x2", d => d)
        .attr("y2", height);
    svg.append("g")
        .attr("class", "grid")
        .selectAll("line")
        .data(d3.range(0, Math.max(width, height), gridSize))
        .enter().append("line")
        .attr("x1", 0)
        .attr("y1", d => d)
        .attr("x2", width)
        .attr("y2", d => d);

    // Draw links
    const link = svg.append("g")
        .attr("stroke", "#999")
        .attr("stroke-opacity", 0.6)
        .selectAll("line")
        .data(edges)
        .enter().append("line")
        .attr("stroke-width", 1)
        .attr("x1", d => nodes[d.source].x)
        .attr("y1", d => nodes[d.source].y)
        .attr("x2", d => nodes[d.target].x)
        .attr("y2", d => nodes[d.target].y);

    // Draw nodes
    const node = svg.append("g")
        .attr("stroke", "#fff")
        .attr("stroke-width", 1.5)
        .selectAll("circle")
        .data(nodes)
        .enter().append("circle")
        .attr("r", 5)
        .attr("fill", "red")
        .attr("cx", d => d.x)
        .attr("cy", d => d.y)
        .attr("class", "node");

    node.append("title")
        .text(d => d.id);

    node.on("click", function(event, d) {
        d.fixed = !d.fixed;
        d3.select(this).classed("fixed", d.fixed);
        if (!d.fixed) {
            d.fx = null;
            d.fy = null;
        }
    });
}

function loadAlgorithmFromFile(file) {
    const reader = new FileReader();
    reader.onload = (event) => {
        const scriptContent = event.target.result;
        const script = document.createElement("script");
        script.type = "module";
        script.textContent = `
            import { default as customAlgorithm } from 'data:text/javascript;base64,${btoa(scriptContent)}';
            window.customAlgorithm = customAlgorithm;
        `;
        document.body.appendChild(script);

        setTimeout(() => {
            if (window.customAlgorithm) {
                uploadedAlgorithm = window.customAlgorithm;
                console.log("Custom algorithm loaded:", uploadedAlgorithm);
            } else {
                console.error("Failed to load custom algorithm.");
            }
        }, 100);
    };
    reader.readAsText(file);
}

document.getElementById("upload-input").addEventListener("change", (event) => {
    const file = event.target.files[0];
    if (file) {
        loadAlgorithmFromFile(file);
    }
});

document.getElementById("algorithm-select").addEventListener("change", (event) => {
    const algorithm = event.target.value;
    updateControlVisibility(algorithm);
});

const graph_id = new URLSearchParams(window.location.search).get('id');
fetch(`http://localhost:8080/api/graph/${graph_id}`)
    .then(response => response.json())
    .then(data => {
        const algorithm = "process_force_atlas2";
        const iterations = 1000;
        const gravity = 0.1;
        const scalingRatio = 10.0;
        updateControlVisibility(algorithm);
        run(data.content, algorithm, iterations, gravity, scalingRatio);
    });

document.getElementById("refresh-button").addEventListener("click", () => {
    const algorithm = document.getElementById("algorithm-select").value;
    const iterations = parseInt(document.getElementById("iterations-input").value) || 1000;
    const gravity = parseFloat(document.getElementById("gravity-input").value) || 0.1;
    const scalingRatio = parseFloat(document.getElementById("scaling-ratio-input").value) || 10.0;
    const graph_id = new URLSearchParams(window.location.search).get('id');
    fetch(`http://localhost:8080/api/graph/${graph_id}`)
        .then(response => response.json())
        .then(data => {
            run(data.content, algorithm, iterations, gravity, scalingRatio);
        });
});

// Initialize control visibility based on the default selected algorithm
updateControlVisibility(document.getElementById("algorithm-select").value);