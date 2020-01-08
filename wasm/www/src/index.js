import './styles.css';

// Wasm modules must be imported asynchronously.
let backendPromise = import("./backend.js");

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
        document.getElementById('mainPlot'), years, ticksX, updateTooltip);

    let backend = await backendPromise;
    let handle = await backend.loadFile();
    document.getElementById('demo').onclick = function () {
        let trajectories = handle.pairwise_trajectories(
            [13431, 13431, 13431, 13431, 13431, 13431, 13431,],
            [4722, 6710, 23815, 12995, 19844, 14661, 8848]
        );

        let metaData = [
            { mainWord: 'broadcast', otherWord: 'radio', colorIndex: 0, },
            { mainWord: 'broadcast', otherWord: 'television', colorIndex: 1, },
            { mainWord: 'broadcast', otherWord: 'harvested', colorIndex: 2, },
            { mainWord: 'broadcast', otherWord: 'propagated', colorIndex: 3, },
            { mainWord: 'broadcast', otherWord: 'strewn', colorIndex: 4, },
            { mainWord: 'broadcast', otherWord: 'sowing', colorIndex: 5, },
        ];

        for (let i = 0; i < metaData.length; i += 1) {
            let { mainWord, otherWord, colorIndex } = metaData[i];
            let values = trajectories.subarray(i * 209, (i + 1) * 209);
            mainPlot.plotLine(
                values,
                colorIndex,
                0,
                {
                    mainWord,
                    description: mainWord + ' : ' + otherWord
                }
            );
        }
    }

    let suggestions = document.getElementsByClassName('suggestion');
    for (let i = 0; i < suggestions.length; i += 1) {
        suggestions[i].addEventListener('click', e => {
            e.target.classList.toggle('active');
        });
    }

    function updateTooltip(tooltip, line, indexX) {
        tooltip.classList.add('waiting');
        tooltip.querySelector('.year').innerText = years[indexX];
        tooltip.querySelector('.lineDescription').innerText = line.payload.description;
        tooltip.querySelector('.mainWord').innerText = line.payload.mainWord;

        // TODO: defer (setTimeout(..., 0)) calculation of nearest neighbors;
        // when done and not canceled, write them to tooltip.
        // (Unless results are cached, in which case we immediately display
        // them without setting a timeout.)
    }
}())
