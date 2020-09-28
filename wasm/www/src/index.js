import './styles.css';

import metaDataFile from "../assets/googlebooks_metadata_1800to2008_vocabsize30000.bin";
//import FaceBookIcon from './facebook-icon.png'
//import TwitterIcon from './facebook-icon.png'

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
    let ticksX = [];
    for (let year = 1800; year <= 2008; year += 1) {
        years.push(year);
        if (year % 20 === 0) {
            ticksX.push(year);
        }
    }

    let currentWord = ''; // Invariant: `currentWord` is always either '' or a valid word from the vocabulary.
    let manualComparisons = [];
    let manualComparisonIds = [];

    let legend = document.getElementById('mainLegend');
    let suggestedComparisonItems = document.getElementById('suggestedComparisons').querySelectorAll('li');
    let manualComparisonItems = document.getElementById('manualComparisons').querySelectorAll('li');
    let suggestedComparisonIds = null;
    let manualComparisonInputs = [];
    let manualComparisonRemoveButtons = [];
    let allComparisonItems = [...suggestedComparisonItems, ...manualComparisonItems];

    let inputWidthMeasure = document.querySelector('.inputWidthMeasure');

    let updateTooltip = (function () {
        let tooltip = document.getElementById('tooltipTemplate');
        let tooltipContent = tooltip.querySelector('.tooltipContent');
        let yearPlaceholder = tooltip.querySelector('.year');
        let word1Placeholder = tooltip.querySelector('.word1');
        let word2Placeholder = tooltip.querySelector('.word2>a');
        let relatedPlaceholders = [];
        let relatedRemoveButtons = [];
        let relatedTimeout = null;
        let relatedCache = [{}, {}];
        let relatedCacheFilling = [0, 0];
        let relatedCacheGeneration = 0;
        const MAX_CACHE_FILLING = 1024;

        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                updatePlot(el.innerText, null);
            });
        });
        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                updatePlot(el.innerText, null);
            });
        });
        word2Placeholder.addEventListener('click', ev => {
            ev.preventDefault();
            word2Placeholder.blur();
            updatePlot(word2Placeholder.innerText, null);
        });

        return function (tooltip, line, indexX) {
            clearTimeout(relatedTimeout);
            let payload = line.payload;
            yearPlaceholder.innerText = years[indexX];
            word1Placeholder.innerText = payload.word1;
            word2Placeholder.innerText = payload.word2;

            // TODO: look up word1 and word2 in cache independently.
            let cacheKey = payload.word1Id + '-' + payload.word2Id + '-' + indexX;
            let cachedCurrent = relatedCache[relatedCacheGeneration][cacheKey];
            let cached = cachedCurrent || relatedCache[1 - relatedCacheGeneration][cacheKey];
            if (typeof cached !== 'undefined') {
                cached.forEach((r, i) => {
                    relatedPlaceholders[i].innerText = metaData.vocab[r];
                });
                tooltipContent.classList.remove('waiting');

                if (typeof cachedCurrent === 'undefined') {
                    // Entry was found in old generation of the cache. Add it also to the current
                    // generation so that it continues to stay cached for a while. If this would
                    // overflow the current generation of the cache then flip generation instead.
                    if (relatedCacheFilling[relatedCacheGeneration] === MAX_CACHE_FILLING) {
                        relatedCacheGeneration = 1 - relatedCacheGeneration;
                        relatedCache[relatedCacheGeneration] = {};
                        relatedCacheFilling[relatedCacheGeneration] = 0;
                    }
                    relatedCache[relatedCacheGeneration][cacheKey] = cached;
                    relatedCacheFilling[relatedCacheGeneration] += 1;
                }
            } else {
                tooltipContent.classList.add('waiting');
                relatedTimeout = setTimeout(() => {
                    let related = handle.most_related_to_at_t([payload.word1Id, payload.word2Id], indexX, 7);
                    related.forEach((r, i) => {
                        relatedPlaceholders[i].innerText = metaData.vocab[r];
                    });
                    tooltipContent.classList.remove('waiting');

                    if (relatedCacheFilling[relatedCacheGeneration] == MAX_CACHE_FILLING) {
                        relatedCacheGeneration = 1 - relatedCacheGeneration;
                        relatedCache[relatedCacheGeneration] = {};
                        relatedCacheFilling[relatedCacheGeneration] = 0;
                    }
                    relatedCache[relatedCacheGeneration][cacheKey] = related;
                    relatedCacheFilling[relatedCacheGeneration] += 1;
                }, 0);
            }
        };
    }());

    let lineMouseover = function (lineId) {
        allComparisonItems[lineId].classList.add('hovering');
    };

    let lineMouseout = function (lineId) {
        allComparisonItems[lineId].classList.remove('hovering');
    };

    const mainPlot = Plotter.createPlot(
        document.getElementById('mainPlot'), years, ticksX, updateTooltip,
        document.getElementById('tooltipTemplate'), lineMouseover, lineMouseout);

    document.getElementById('mainLegend').querySelectorAll('ul').forEach(
        element => element.addEventListener('mouseout', () => mainPlot.lineToFront())
    );

    allComparisonItems.forEach((element, index) => {
        element.addEventListener('mouseover', () => { mainPlot.lineToFront(index); mainPlot.hoverLine(index) });
        element.addEventListener('mouseout', () => mainPlot.unhoverLine(index));
        element.addEventListener('click', () => mainPlot.setMainLine(index));

        const legendLink = element.querySelector('a');
        if (legendLink) {
            legendLink.addEventListener('click', ev => {
                ev.preventDefault();
                legendLink.blur();
                updatePlot(legendLink.innerText, null);
            });
        }

        const inputs = element.querySelectorAll('input');
        if (inputs.length !== 0) {
            const [otherWordInput, removeButton] = inputs;
            let manualIndex = manualComparisonInputs.length;
            manualComparisonInputs.push(otherWordInput);
            manualComparisonRemoveButtons.push(removeButton);

            let inputEventHandler = () => manualComparisonChanged(otherWordInput, manualIndex);
            otherWordInput.onkeydown = inputEventHandler;
            otherWordInput.onchange = inputEventHandler;
            otherWordInput.onclick = inputEventHandler;
            otherWordInput.onblur = inputEventHandler;

            removeButton.onclick = () => removeManualComparison(manualIndex);

            if (manualIndex === 0) {
                otherWordInput.style.width = '0';
                removeButton.style.display = 'none';
            } else {
                element.style.display = 'none';
            }
        }
    });

    let [handle, metaData] = await Promise.all([
        backendPromise.then(backend => backend.loadFile()),
        fetch(metaDataFile).then(file => file.json())
    ]);
    document.getElementById('downloadProgressPane').style.display = 'none';
    document.querySelector('.app').style.display = 'block';

    let inverseVocab = {};
    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);


    let wordInput = document.querySelector('.wordInput');
    let wordInputError = document.querySelector('.wordInputError');
    // We listen to several events to make the UI snappier. For example,
    // `onkeydown` fires earlier than `onchange` but it misses some changes such
    // as "right-click --> paste". Listening to several events does not
    // significantly increase  computational cost because the event handler
    // performs expensive calculations only if anything actually changed.
    wordInput.onkeydown = wordChanged;
    wordInput.onchange = wordChanged;
    wordInput.onclick = wordChanged;
    wordInput.onblur = wordChanged;

    let shareFacebookButton = document.getElementById('shareFacebookButton');
    shareFacebookButton.onclick = shareFaceBook;

    let shareTwitterButton = document.getElementById('shareTwitterButton');
    shareTwitterButton.onclick = shareTwitter;

    let showUrlButton = document.getElementById('showUrlButton');
    //console.log("here", showUrlButton);
    showUrlButton.onclick = showUrl;

    let dynamicMainLegendDOMs = [];//to keep track of dynamically added entries

    window.addEventListener('popstate', on_popstate);
    setTimeout(() => {
        on_popstate();
        wordInput.selectionStart = wordInput.selectionEnd = wordInput.value.length;
        wordInput.focus();
        if (currentWord === '') {
            // Explicitly clear plot so that prompt becomes visible.
            mainPlot.clear();
        }
    }, 0);

    let colorsAvail = ['color6', 'color7', 'color8', 'color9'];

    function shareFaceBook() {
        //console.log("//TODO: copy current link to url2");
        window.open(
            'https://www.facebook.com/sharer/sharer.php?u=' + encodeURIComponent(location.href),
            'facebook-share-dialog',
            'width=626,height=436');
    }

    function shareTwitter() {
        //console.log("//TODO: copy current link to url");
        window.open(
            "https://twitter.com/intent/tweet?text=check this out! -> " + encodeURIComponent(location.href),
            'facebook-share-dialog',
            'width=626,height=436');
    }

    function showUrl() {
        //console.log("//TODO: copy show this url to user");
        alert("copy this link to share -> ".concat(location.href.toString()));
    }

    function on_popstate() {
        let newMainWord = "";
        let newManualComparisons = [];
        for (let url_component of window.location.hash.substr(1).split("&")) {
            let [key, value] = url_component.split("=");
            if (key === "w") {
                newMainWord = decodeURIComponent(value);
            } else if (key === "o" && value !== "") {
                newManualComparisons = value.split("+").map(decodeURIComponent);
            }
        }

        updatePlot(newMainWord, newManualComparisons, true);
    }

    function wordChanged() {
        // Wait for next turn in JS executor to let change take effect.
        setTimeout(() => updatePlot(wordInput.value.trim(), null), 0);
    }

    function manualComparisonChanged(inputField, index) {
        // Wait for next turn in JS executor to let change take effect.
        setTimeout(() => {
            let otherWord = inputField.value.trim();

            // Make a *copy* of the array so that `updatePlot` can check if anything changed.
            let newManualComparisons = [...manualComparisons];
            if (index >= newManualComparisons.length - 1 && otherWord === '') {
                // Last nonempty input box was emptied out. Remove the word. The input box
                // will still stick around anyway.
                newManualComparisons.splice(index, 1);
            } else if (index < newManualComparisons.length) {
                newManualComparisons[index] = otherWord;
            } else {
                newManualComparisons.push(otherWord);
            }
            updatePlot(null, newManualComparisons);
            mainPlot.setMainLine(suggestedComparisonItems.length + index);
        }, 0);
    }

    function removeManualComparison(index) {
        // Make a *copy* of the array so that `updatePlot` can check if anything changed.
        let newManualComparisons = [...manualComparisons];
        if (index < newManualComparisons.length) {
            newManualComparisons.splice(index, 1); // Removes the element.
            updatePlot(null, newManualComparisons);
        }
    }

    function updatePlot(newMainWord, newManualComparisons, suppress_save_state = false) {
        let mainWordChanged = false;
        let manualComparisonsChanged = false;

        if (newMainWord !== null) {
            if (wordInput.value.trim() !== newMainWord) {
                wordInput.value = newMainWord;
            }
            let newMainWordId = inverseVocab[newMainWord];
            if (newMainWord === '' || typeof newMainWordId !== 'undefined') {
                wordInput.classList.remove('invalid');
                wordInputError.style.display = 'none';
                if (newMainWord !== currentWord) {
                    mainWordChanged = true;
                    currentWord = newMainWord;
                    suggestedComparisonIds = handle.largest_changes_wrt(newMainWordId, suggestedComparisonItems.length, 2, 2);
                }
            } else {
                // Out of vocabulary word entered. Treat as if `currentWord` did not change. 
                // We may still want to update the plot in case `manualComparisons` changed.
                wordInput.classList.add('invalid');
                wordInputError.style.display = 'inline-block';
            }
        }

        if (newManualComparisons !== null) {
            let newManualComparisonIds = [];
            if (newManualComparisons.length > manualComparisonItems.length) {
                newManualComparisons.splice(manualComparisonItems.length); // Removes everything that flows over.
            }

            // Update input boxes in legend.
            for (let i = 0; i < newManualComparisons.length; i += 1) {
                let otherWord = newManualComparisons[i];
                let otherWordId = inverseVocab[otherWord];
                newManualComparisonIds.push(otherWordId);

                if (i >= manualComparisons.length || manualComparisons[i] !== otherWord) {
                    manualComparisonsChanged = true;
                    if (typeof otherWordId === 'undefined') {
                        manualComparisonInputs[i].classList.add('invalid');
                        manualComparisonInputs[i].setAttribute('title', 'word not found');
                        manualComparisonInputs[i].parentElement.removeAttribute('title');
                    } else {
                        manualComparisonInputs[i].classList.remove('invalid');
                        manualComparisonInputs[i].removeAttribute('title');
                        manualComparisonInputs[i].parentElement.setAttribute(
                            'title', 'Click and move mouse across diagram to explore further.'
                        );
                    }
                    manualComparisonItems[i].style.display = 'list-item';
                    manualComparisonRemoveButtons[i].style.display = 'inline';
                    if (manualComparisonInputs[i].value.trim() !== otherWord) {
                        manualComparisonInputs[i].value = otherWord;
                    }
                    inputWidthMeasure.textContent = otherWord;
                    manualComparisonInputs[i].style.width = inputWidthMeasure.offsetWidth + 'px';
                }
            }

            if (newManualComparisons.length !== manualComparisons.length) {
                manualComparisonsChanged = true;

                if (newManualComparisons.length < manualComparisonItems.length) {
                    // There's still room for additional manual comparisons, so show an empty input box.
                    manualComparisonItems[newManualComparisons.length].style.display = 'list-item';
                    manualComparisonInputs[newManualComparisons.length].value = '';
                    manualComparisonInputs[newManualComparisons.length].style.width = '0';
                    manualComparisonInputs[newManualComparisons.length].classList.remove('invalid');
                    manualComparisonInputs[newManualComparisons.length].setAttribute(
                        'title', 'Enter a secondary word here.'
                    );
                    manualComparisonInputs[newManualComparisons.length].parentElement.removeAttribute('title');
                    manualComparisonRemoveButtons[newManualComparisons.length].style.display = 'none';

                    // Remove all input boxes below.
                    for (let i = newManualComparisons.length + 1; i < manualComparisonItems.length; i += 1) {
                        manualComparisonItems[i].style.display = 'none';
                    }
                }
            }

            manualComparisons = newManualComparisons;
            manualComparisonIds = newManualComparisonIds;
        }

        // Do the expensive stuff only if anything actually changed. This allows us to
        // attach this function on lots of events to catch changes as early as possible
        // without firing multiple times on the same change.
        if (mainWordChanged || manualComparisonsChanged) {
            mainPlot.clear();

            if (currentWord === '') {
                legend.style.display = 'none';
                if (!suppress_save_state) {
                    history.pushState(null, "The Linguistic Time Capsule", "#");
                }
                return;
            }

            if (!suppress_save_state) {
                let stateUrl = "#v=0&c=en&w=" + encodeURIComponent(currentWord);
                if (manualComparisons.length != 0) {
                    stateUrl = stateUrl + "&o=" + manualComparisons.map(encodeURIComponent).join("+");
                }
                history.pushState(null, "The Linguistic Time Capsule: " + currentWord, stateUrl);
            }

            legend.style.display = 'block';
            allComparisonItems.forEach(el => {
                el.classList.remove('hovering');
                el.firstElementChild.textContent = currentWord;
            });

            let otherWordIds = [...suggestedComparisonIds];
            let comparisonColors = [];
            for (let i = 0; i < otherWordIds.length; i += 1) {
                comparisonColors.push(i);
            }
            manualComparisonIds.forEach((id, index) => {
                if (typeof id !== 'undefined') {
                    otherWordIds.push(id)
                    comparisonColors.push(suggestedComparisonIds.length + index);
                }
            });

            let mainWordId = inverseVocab[currentWord];
            let wordIdRepeated = Array(otherWordIds.length).fill(mainWordId);
            let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWordIds);
            let trajectoryLength = concatenatedTrajectories.length / otherWordIds.length;

            otherWordIds.forEach((otherWordId, index) => {
                let otherWord = metaData.vocab[otherWordId];
                mainPlot.plotLine(
                    concatenatedTrajectories.subarray(index * trajectoryLength, (index + 1) * trajectoryLength),
                    comparisonColors[index],
                    0,
                    {
                        word1: currentWord,
                        word2: otherWord,
                        word1Id: mainWordId,
                        word2Id: otherWordId,
                    },
                    false,
                    '"' + currentWord + '" ↔ "' + otherWord + '"\n(click on line to explore relationship)'
                );

                if (index < suggestedComparisonItems.length) {
                    allComparisonItems[index].firstElementChild.nextElementSibling.textContent = otherWord;
                }
            });
        }
    }
}())
