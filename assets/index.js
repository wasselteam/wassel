const app = document.querySelector("#app");

const refreshStats = async () => {
    const response = await fetch("/stats");
    const stats = await response.json();
    app.replaceChildren(statsElement(stats));
};

refreshStats();

setInterval(refreshStats, 1000);

function statsElement(stats) {
    const { cpuUsage, memory, virtualMemory, startTime } = stats.system;

    const startDate = new Date(startTime * 1000);

    const e = document.createElement("div");
    e.innerHTML = `
        <h2>System statistics</h2>
        <p>CPU: ${cpuUsage}%</p>
        <p>Memory: ${humanreadableSize(memory)}</p>
        <p>Virtual memory: ${humanreadableSize(virtualMemory)}</p>
        <p>Started: ${startDate}</p>
        <h2>Loaded plugins</h2>
        <ul>
            ${stats.plugins.map((p) => pluginElement(p).outerHTML).join("")}
        </ul>
    `;

    return e;
}

function pluginElement({ id, name, version, description, endpoint }) {
    const e = document.createElement("div");
    e.innerHTML = `
        <p>${endpoint}: ${id}</p>
        <p>name: ${name}</p>
        <p>version: ${version}</p>
    `;

    if (description) {
        e.innerHTML += `<p>description: ${description}</p>`;
    }

    e.style = `
        border-top: solid 1px;
    `;

    return e;
}

const KILOBYTE = 1024;
const MEGABYTE = KILOBYTE * 1024;
const GIGABYTE = MEGABYTE * 1024;
const TERABYTE = GIGABYTE * 1024;

function humanreadableSize(bytes) {
    const round = (num) => Math.round(num * 10) / 10;
    const digits = Math.log10(bytes) + 1;
    if (digits < 3) {
        return `${round(bytes)} bytes`;
    } else if (digits < 6) {
        return `${round(bytes / KILOBYTE)}KB`;
    } else if (digits < 9) {
        return `${round(bytes / MEGABYTE)}MB`;
    } else if (digits < 12) {
        return `${round(bytes / GIGABYTE)}GB`;
    } else {
        return `${round(bytes / TERABYTE)}TB`;
    }
}

// TODO: Reactive components:
//       Something like thit:
//       ```
//       class StatsElement extends HTMLElement {
//           static observedAttributes = ["stats"];
//           connectedCallback() {
//               const shadow = this.attachShadow({ mode: "open" });
//               shadow.innerHTML = `
//                   <h2>System statistics</h2>
//                   <p>CPU: 0%</p>
//                   <p>Memory: ${humanreadableSize(0)}</p>
//                   <p>Virtual memory: ${humanreadableSize(0)}</p>
//                   <p>Started: unknown</p>
//                   <h2>Loaded plugins</h2>
//                   <ul></ul>
//               `
//           }
//           attributeChangedCallback(name, _, newValue) {
//               this["$" + name] = newValue
//           }
//       }
//       ```
