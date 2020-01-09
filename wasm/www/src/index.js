import './styles.css';

import metaDataFile from "../assets/googlebooks_metadata_1800to2008_vocabsize30000";

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


    let updateTooltip = (function () {
        let tooltip = document.getElementById('tooltipTemplate');
        let yearPlaceholder = tooltip.querySelector('.year');
        let word1Placeholder = tooltip.querySelector('.word1');
        let word2Placeholder = tooltip.querySelector('.word2');
        let relatedPlaceholders = [];
        let relatedTimeout = null;
        let relatedCache = {};
        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                addPairwiseTrajectory(el.innerText, word1Placeholder.innerText);
            });
        });
        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                addPairwiseTrajectory(el.innerText, word2Placeholder.innerText);
            });
        });

        return function (tooltip, line, indexX) {
            clearTimeout(relatedTimeout);
            let payload = line.payload;
            yearPlaceholder.innerText = years[indexX];
            word1Placeholder.innerText = payload.word1;
            word2Placeholder.innerText = payload.word2;

            let cacheKey = payload.word1Id + '-' + payload.word2Id + '-' + indexX;
            let cached = relatedCache[cacheKey];
            if (typeof (cached) !== 'undefined') {
                cached.forEach((r, i) => {
                    relatedPlaceholders[i].innerText = metaData.vocab[r];
                });
            } else {
                relatedPlaceholders.forEach(e => e.innerText = ' ');
                relatedPlaceholders[0].innerText = '[calculating ...]';

                // TODO: look up word1 and word2 in cache independently.
                relatedTimeout = setTimeout(() => {
                    let related = handle.most_related_to_at_t([payload.word1Id, payload.word2Id], indexX, 7);
                    relatedCache[cacheKey] = related;
                    related.forEach((r, i) => {
                        relatedPlaceholders[i].innerText = metaData.vocab[r];
                    });
                },
                    0);
            }
        };
    }());

    const mainPlot = Plotter.createPlot(
        document.getElementById('mainPlot'), years, ticksX, updateTooltip,
        document.getElementById('tooltipTemplate'));

    let backend = await backendPromise;
    let handle = await backend.loadFile();
    let metaData = await (await fetch(metaDataFile)).json();
    let inverseVocab = {};
    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);
    let colorIndex = 0;

    document.getElementById('demo').onclick = function () {
        let word1 = document.getElementById('word1').value;
        let word2 = document.getElementById('word2').value;
        addPairwiseTrajectory(word1, word2);
    };

    function addPairwiseTrajectory(word1, word2) {
        let word1Id = inverseVocab[word1];
        let word2Id = inverseVocab[word2];

        if (typeof word1Id !== 'undefined' && typeof word2Id !== 'undefined') {
            let trajectory = handle.pairwise_trajectories([word1Id], [word2Id]);

            mainPlot.plotLine(
                trajectory,
                colorIndex,
                0,
                {
                    word1,
                    word2,
                    word1Id,
                    word2Id,
                    description: word1 + ' : ' + word2
                },
                true
            );

            colorIndex = (colorIndex + 1) % 7;
        }
    }
}())
