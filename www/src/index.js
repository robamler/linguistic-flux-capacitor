import './styles.css';

// Wasm modules must be imported asynchronously.
import("./backend.js")
    .catch(e => console.error("Error importing `backend.js`:", e));

(async function () {
    const Plotter = await import('./plotting/main.mjs');
    if (document.readyState === 'loading') {
        await new Promise(function (resolve, _reject) {
            window.addEventListener('DOMContentLoaded', resolve);
        });
    }

    let years = [];
    let pointsY1 = [];
    let pointsY2 = [];
    let ticksX = [];
    for (let year = 1800; year <= 2008; year += 1) {
        years.push(year);
        if (year % 20 === 0) {
            ticksX.push(year);
        }

        pointsY1.push(0.3 * Math.sin(0.1 * year));
        pointsY2.push(0.2 * Math.sin(0.2 * year) + 0.002 * (year - 1900));
    }

    const mainPlot = Plotter.createPlot(
        document.getElementById('mainPlot'), years, ticksX, updateTooltip, 0.3);

    mainPlot.plotLine(pointsY1, 0, 0, 'line1');
    mainPlot.plotLine(pointsY2, 1, 0, 'line2');

    let suggestions = document.getElementsByClassName('suggestion');
    for (let i = 0; i < suggestions.length; i += 1) {
        suggestions[i].addEventListener('click', e => {
            e.target.classList.toggle('active');
        });
    }

    function updateTooltip(tooltip, line, indexX) {
        tooltip.classList.add('waiting');
        tooltip.querySelector('.year').innerText = years[indexX];
        tooltip.querySelector('.mainWord').innerText = line.payload;

        // TODO: defer (setTimeout(..., 0)) calculation of nearest neighbors;
        // when done and not canceled, write them to tooltip.
        // (Unless results are cached, in which case we immediately display
        // them without setting a timeout.)
    }
}())
