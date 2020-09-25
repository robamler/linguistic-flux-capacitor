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
    let mustIncludeWordList = [];

    let mainLegend = document.getElementById('mainLegend');
    let mainLegendItems = mainLegend.querySelectorAll('li');

    let updateTooltip = (function () {
        let tooltip = document.getElementById('tooltipTemplate');
        let tooltipContent = tooltip.querySelector('.tooltipContent');
        let yearPlaceholder = tooltip.querySelector('.year');
        let word1Placeholder = tooltip.querySelector('.word1');
        let word2Placeholder = tooltip.querySelector('.word2>a');
        let relatedPlaceholders = [];
        let relatedRemoveButtons = [];
        let relatedTimeout = null;
        let relatedCache = {};
        tooltip.querySelectorAll('.suggestion.left>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                exploreWord(el.innerText, null);
            });
        });
        tooltip.querySelectorAll('.suggestion.right>a').forEach(el => {
            relatedPlaceholders.push(el);
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                exploreWord(el.innerText, null);
            });
        });
        word2Placeholder.addEventListener('click', ev => {
            ev.preventDefault();
            word2Placeholder.blur();
            exploreWord(word2Placeholder.innerText, null);
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
            exploreWord(legendLink.innerText, null);
        });
    });

    let [handle, metaData] = await Promise.all([
        backendPromise.then(backend => backend.loadFile()),
        fetch(metaDataFile).then(file => file.json())
    ]);
    document.getElementById('downloadProgressPane').style.display = 'none';
    document.getElementById('app').style.display = 'block';

    let inverseVocab = {};
    metaData.vocab.forEach((word, index) => inverseVocab[word] = index);


    let wordInput = document.querySelector('.wordInput');
    // TODO: explain
    wordInput.onkeydown = wordChanged;
    wordInput.onchange = wordChanged;

    let mustIncludeInput = document.querySelector('.mustIncludeInput');


    let pinWordButton = document.getElementById('pinWordButton');
    pinWordButton.onclick = pinWord;

    let shareFacebookButton = document.getElementById('shareFacebookButton');
    shareFacebookButton.onclick = shareFaceBook;

    let shareTwitterButton = document.getElementById('shareTwitterButton');
    shareTwitterButton.onclick = shareTwitter;

    let showUrlButton = document.getElementById('showUrlButton');
    //console.log("here", showUrlButton);
    showUrlButton.onclick = showUrl;

    let dynamicMainLegendDOMs = [];//to keep track of dynamically added entries


    let DEBUG_history_count = 0;
    window.addEventListener('popstate', on_popstate);

    wordInput.focus();
    history.pushState(0, "some useless title", "");

    let colorsAvail = ['color6', 'color7', 'color8', 'color9'];

    setTimeout(on_popstate, 0);

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
        DEBUG_history_count--;
        let mw = "";
        let mi = [];
        for (let url_component of window.location.hash.substr(1).split("&")) {
            let [key, value] = url_component.split("=");
            if (key === "w") {
                mw = decodeURIComponent(value);
            } else if (key === "o" && value !== "") {
                mi = value.split("+").map(decodeURIComponent);
            }
        }

        if (mw == "") {
            mainLegend.style.visibility = 'hidden';
            mainPlot.clear();
            restoreState(mw, []);
            return;
        }

        restoreState(mw, mi);
    }

    function restoreState(savedMainWord, savedOtherWords) {
        wordInput.value = savedMainWord;
        exploreWord(savedMainWord, savedOtherWords, true);
    }

    function wordChanged() {
        console.log("word changed: main input changed to: ", wordInput.value);
        // Wait for next turn in JS executor to let change take effect.
        setTimeout(() => exploreWord(wordInput.value, null), 0);
    }

    function pinWord() {
        //this function is called when the pin word button is called
        var word = mustIncludeInput.value;
        if (word == "") {
            return;
        }
        if (mustIncludeWordList.length == 4) {
            alert("must included word approached threshhold");
            return;
        }
        let wordId = inverseVocab[word];
        if (typeof wordId === 'undefined') {
            mustIncludeInput.classList.add('invalid');
            return;
        }
        else {
            mustIncludeInput.classList.remove('invalid');
        }

        // Make a *copy* of the array and append the new word.
        let newMustIncludeWordList = [...mustIncludeWordList, word];
        exploreWord(null, newMustIncludeWordList);
        mustIncludeInput.value = '';
    }

    function removeWordButtonCallback(removeWordButton) {
        var word2Remove = removeWordButton.getAttribute("name");
        //remove word from must included list
        mustIncludeWordList = mustIncludeWordList.filter(e => e !== word2Remove);
        //notify that must included list changed
        exploreWord(wordInput.value, null);
    }


    function assembleMainLegendDOM(colorIndex) {
        /*return a li object that is similar to that of the original 6 li DOM obj in main legend*/
        //var colorString = colorsAvail[colorsUsed];
        var html = '<li id=\'dynamicLiObj\' class=\'_COLORNUM_\'><span></span> ↔ <a href=\'#\'></a>&nbsp&nbsp<button id=\'rmBtn6\' class=\"tooltipContent removeWordButton\" name="na" style="position: absolute; right: 0;">x</button></li>'
        if (colorIndex == null) {
            html = html.replace("_COLORNUM_", "color6");
        }
        else {
            html = html.replace("_COLORNUM_", colorsAvail[colorIndex]);
        }
        //console.log("Assembled: ", html);
        var template = document.createElement('template');
        template.innerHTML = html;
        var el = template.content.firstChild;
        el.querySelectorAll('.removeWordButton').forEach(el => {
            //relatedRemoveButtons.push(el);
            el.setAttribute("name", "defaultRemoval");
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                removeWordButtonCallback(el);
            });
        });
        return el;
    }



    function addSlotToMainLegend(colorIndex) {
        /*add a new empty DOM li to main legend ul, the content is set in exploreword*/
        var ul = document.getElementById("plotUL");
        var el = assembleMainLegendDOM(colorIndex);
        ul.append(el);
        dynamicMainLegendDOMs.push(el);
        //force refresh main plot;
        mainLegend = document.getElementById('mainLegend');
        mainLegendItems = mainLegend.querySelectorAll('li');
    }

    function removeSlotFromMainLegend() {
        /*in case other operations is needed in the future*/
        mainLegendItems.pop();
    }

    function cleanMainLegend() {
        /*remove all dynamically added slot from main legend*/
        var numToIter = dynamicMainLegendDOMs.length;

        for (let i = numToIter; i > 0; i--) {
            var victim = dynamicMainLegendDOMs[i - 1];
            victim.parentNode.removeChild(victim);
        }
        //force refresh main plot;
        mainLegend = document.getElementById('mainLegend');
        mainLegendItems = mainLegend.querySelectorAll('li');
        //notify all dynamic objects are free now
        dynamicMainLegendDOMs.length = 0;
    }

    function saveHistoryState() {
        //save current state of webpage
    }

    function exploreWord(newMainWord, newMustIncludeWordList, suppress_save_state = false) {
        // Check if either the word or the include list actually changed.
        // If both are still the old ones then we don't do anything here.
        let mainWordChanged = (newMainWord !== null && newMainWord !== currentWord);
        let mustIncludeWordListChanged = (newMustIncludeWordList !== null &&
            newMustIncludeWordList !== mustIncludeWordList);

        if (mainWordChanged || mustIncludeWordListChanged) {
            if (newMainWord !== null) {
                currentWord = newMainWord;
            }
            if (newMustIncludeWordList !== null) {
                mustIncludeWordList = newMustIncludeWordList;
            }

            mainLegendItems.forEach(el => el.classList.remove('hovering'));

            console.log("exploreWord called, word: ", currentWord, " ,mustIncludeWordList: ", mustIncludeWordList);
            //corner case: infinite loop
            if (!suppress_save_state) {
                let stateUrl = "#";
                if (currentWord !== "") {
                    stateUrl = stateUrl + "c=en&w=" + encodeURIComponent(currentWord);
                    if (mustIncludeWordList.length != 0) {
                        stateUrl = stateUrl + "&o=" + mustIncludeWordList.map(encodeURIComponent).join("+");
                    }
                }
                history.pushState(DEBUG_history_count++, "some useless title", stateUrl); // TODO: set title
                //console.log("state pushed, total states: ", DEBUG_history_count);
            }

            if (currentWord == "") {
                //console.log("detected empty in wordchanged");
                mainPlot.clear();
                mustIncludeWordList = [];
                mainLegend.style.visibility = 'hidden';
                return;
            }

            if (mustIncludeWordListChanged) {
                //console.log("detected must include list updating")
                cleanMainLegend();

                var currentLegendLength = mainLegendItems.length;
                let newLegendLength = 6 + mustIncludeWordList.length;
                for (let i = currentLegendLength; i <= newLegendLength; i += 1) {
                    addSlotToMainLegend(i - 6);
                }

                //console.log("rebinding dynamic dom objects to lines(async), items count: ", dynamicMainLegendDOMs.length);
                dynamicMainLegendDOMs.forEach((element, index) => {
                    let actualMainplotIndex = index + 6;
                    element.removeEventListener('mouseover', null);
                    element.removeEventListener('mouseout', null);
                    element.addEventListener('mouseover', () => mainPlot.hoverLine(actualMainplotIndex));
                    element.addEventListener('mouseout', () => mainPlot.unhoverLine(actualMainplotIndex));

                    const legendLink = element.querySelector('a');
                    legendLink.addEventListener('click', ev => {
                        ev.preventDefault();
                        legendLink.blur();
                        mustIncludeWordList.splice(mustIncludeWordList.indexOf(legendLink.innerText), 1);
                        //console.log("going to explore ", legendLink.innerText);
                        cleanMainLegend();
                        exploreWord(legendLink.innerText, null);
                    });
                });
            }

            if (mainWordChanged) {
                let wordId = inverseVocab[currentWord];
                if (typeof wordId === 'undefined') {
                    wordInput.classList.add('invalid');
                } else {
                    wordInput.classList.remove('invalid');
                    if (wordInput.value !== currentWord) {
                        wordInput.value = currentWord;
                    }
                    mainPlot.clear();

                    // `handle.largest_change_wrt` returns a `Uint32Array`, which does not have
                    // a `push` method, so we turn it into a regular JS array.
                    let otherWords = Array.prototype.slice.call(handle.largest_changes_wrt(wordId, 6, 2, 2));
                    for (var i = 0; i < mustIncludeWordList.length; i++) {
                        otherWords.push(inverseVocab[mustIncludeWordList[i]]);
                    }

                    // `handle.pairwise_trajectories` expects two arrays of word IDs, which it
                    // zips into an array of word ID pairs. The frontend currently only supports plots
                    // where the first word of each pair is the same for all pairs, so we have to
                    // copy the ID of the first word for each pair.
                    let wordIdRepeated = Array(otherWords.length).fill(wordId);
                    let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWords);
                    let trajectoryLength = concatenatedTrajectories.length / otherWords.length;

                    otherWords.forEach((otherWordId, index) => {
                        let otherWord = metaData.vocab[otherWordId];
                        //console.log("plotting against ",otherWord);
                        mainPlot.plotLine(
                            concatenatedTrajectories.subarray(index * trajectoryLength, (index + 1) * trajectoryLength),
                            index,
                            0,
                            {
                                word1: currentWord,
                                word2: otherWord,
                                word1Id: wordId,
                                word2Id: otherWordId
                            },
                            false
                        );
                        const legendWordLabel = mainLegendItems[index].firstElementChild;
                        //console.log("legendWodLebl", legendWordLabel);
                        legendWordLabel.textContent = currentWord;

                        legendWordLabel.nextElementSibling.textContent = otherWord;
                        if (legendWordLabel.nextElementSibling.nextElementSibling != null) {
                            //console.log("setting name of button");
                            legendWordLabel.nextElementSibling.nextElementSibling.setAttribute("name", otherWord);
                        }
                    });
                    mainLegend.style.visibility = 'visible';
                }
            }
        }

    }
}())
