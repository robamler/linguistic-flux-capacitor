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
    let currentWord = null;

    const mainLegend = document.getElementById('mainLegend');
    const mainLegendItems = mainLegend.querySelectorAll('li');

    let updateTooltip = (function () {
        let tooltip = document.getElementById('tooltipTemplate');
        let tooltipContent = tooltip.querySelector('.tooltipContent');
        let yearPlaceholder = tooltip.querySelector('.year');
        let word1Placeholder = tooltip.querySelector('.word1');
        let word2Placeholder = tooltip.querySelector('.word2>a');
        let relatedPlaceholders = [];
        let relatedTimeout = null;
        let relatedCache = {};
        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                exploreWord(el.innerText);
            });
        });
        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                exploreWord(el.innerText);
            });
        });
        word2Placeholder.addEventListener('click', ev => {
            ev.preventDefault();
            word2Placeholder.blur();
            exploreWord(word2Placeholder.innerText);
        });


        return function (tooltip, line, indexX) {
            clearTimeout(relatedTimeout);
            let payload = line.payload;
            yearPlaceholder.innerText = years[indexX];
            word1Placeholder.innerText = payload.word1;
            word2Placeholder.innerText = payload.word2;

            // TODO: look up word1 and word2 in cache independently.
            // TODO: clear old entries from cache at some point.
            let cacheKey = payload.word1Id + '-' + payload.word2Id + '-' + indexX;
            let cached = relatedCache[cacheKey];
            if (typeof (cached) !== 'undefined') {
                cached.forEach((r, i) => {
                    relatedPlaceholders[i].innerText = metaData.vocab[r];
                });
                tooltipContent.classList.remove('waiting');
            } else {
                tooltipContent.classList.add('waiting');
                relatedTimeout = setTimeout(() => {
                    tooltipContent.classList.remove('waiting');
                    let related = handle.most_related_to_at_t([payload.word1Id, payload.word2Id], indexX, 7);
                    relatedCache[cacheKey] = related;
                    related.forEach((r, i) => {
                        relatedPlaceholders[i].innerText = metaData.vocab[r];
                    });
                }, 0);
            }
        };
    }());

    let lineMouseover = function (lineId) {
        mainLegendItems[lineId].classList.add('hovering');
    };

    let lineMouseout = function (lineId) {
        mainLegendItems[lineId].classList.remove('hovering');
    };

    const mainPlot = Plotter.createPlot(
        document.getElementById('mainPlot'), years, ticksX, updateTooltip,
        document.getElementById('tooltipTemplate'), lineMouseover, lineMouseout);

    mainLegendItems.forEach((element, index) => {
        element.addEventListener('mouseover', () => mainPlot.hoverLine(index));
        element.addEventListener('mouseout', () => mainPlot.unhoverLine(index));

        const legendLink = element.querySelector('a');
        legendLink.addEventListener('click', ev => {
            ev.preventDefault();
            legendLink.blur();
            exploreWord(legendLink.innerText);
        });
    });

    let backend = await backendPromise;
    let handle = await backend.loadFile();
    let metaData = await (await fetch(metaDataFile)).json();
    let inverseVocab = {};
    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);

    let wordInput = document.querySelector('.wordInput');
    wordInput.onkeydown = wordChanged;
    wordInput.onkeypress = wordChanged;
    wordInput.onchange = wordChanged;

    wordChanged();
    wordInput.focus();

    function wordChanged() {
        // Wait for next turn in JS executor to let change take effect.
        setTimeout(() => exploreWord(wordInput.value), 0);
    }

    function exploreWord(word) {
        if (word !== currentWord) {
            currentWord = word;

            mainLegendItems.forEach(el => el.classList.remove('hovering'));

            let wordId = inverseVocab[word];
            if (typeof wordId === 'undefined') {
                wordInput.classList.add('invalid');
            } else {
                wordInput.classList.remove('invalid');
                if (wordInput.value !== word) {
                    wordInput.value = word;
                }
                mainPlot.clear();
                let otherWords = handle.largest_changes_wrt(wordId, 6, 2, 2);
                let wordIdRepeated = Array(6).fill(wordId);
                let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWords);
                let trajectoryLength = concatenatedTrajectories.length / 6;

                otherWords.forEach((otherWordId, index) => {
                    let otherWord = metaData.vocab[otherWordId];
                    mainPlot.plotLine(
                        concatenatedTrajectories.subarray(index * trajectoryLength, (index + 1) * trajectoryLength),
                        index,
                        0,
                        {
                            word1: word,
                            word2: otherWord,
                            word1Id: wordId,
                            word2Id: otherWordId
                        },
                        false
                    );

                    const legendWordLabel = mainLegendItems[index].firstElementChild;
                    legendWordLabel.textContent = word;
                    legendWordLabel.nextElementSibling.textContent = otherWord;
                });

                mainLegend.style.visibility = 'visible';
            }
        }
    }
}())