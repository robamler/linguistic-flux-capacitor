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
    let currentMustIncludedWord = null;
    let mustIncludeWordList = [];
    let currentMustIncludedWordList = [];
    let mustIncludeListUpdated = false;

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
        /* this code snippets handles remove word button for auto suggested words, if needed, uncomments this
        document.querySelectorAll('.removeWordButton').forEach(el => {
            relatedRemoveButtons.push(el);
            el.setAttribute("name","defaultRemoval");
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                removeWordButtonCallback(el);
            });
        });
        */
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

    let mustIncludeInput = document.querySelector('.mustIncludeInput');
    mustIncludeInput.onkeydown = mustIncludeChanged;
    mustIncludeInput.onkeypress = mustIncludeChanged;
    mustIncludeInput.onchange = mustIncludeChanged;

    let pinWordButton = document.getElementById('pinWordButton');
    pinWordButton.onclick = pinWord;

    wordChanged();
    wordInput.focus();


    
    function wordChanged() {
        // Wait for next turn in JS executor to let change take effect.
        setTimeout(() => exploreWord(wordInput.value, currentMustIncludedWord), 0);
    }


    function mustIncludeChanged() {
        // this function is binded to change in must include input box
        // uncomment next line for real-time update of plot
        //setTimeout(() => exploreWord(wordInput.value, mustIncludeInput.value), 0);
        mustIncludeListUpdated = true;
    }

    function pinWord(){
    	//this function is called when the pin word button is called
        var word = mustIncludeInput.value;
        //console.log('pin this word : ', word);
        mustIncludeWordList.push(word);
        //console.log(mustIncludeWordList);
        exploreWord(wordInput.value, mustIncludeWordList);
        mustIncludeInput.value = '';
    }

    function removeWordButtonCallback(removeWordButton){
        //console.log("//TODO: remove word ".concat(removeWordButton.getAttribute("name")));
        var word2Remove = removeWordButton.getAttribute("name");
        //remove word from must included list
        mustIncludeWordList = mustIncludeWordList.filter(e => e !== word2Remove);
        //notify that must included list changed
        mustIncludeListUpdated = true;
        exploreWord(wordInput.value, mustIncludeWordList);
    }


    
    function assembleMainLegendDOM(){
    	/*return a li object that is similar to that of the original 6 li DOM obj in main legend*/
    	var id = 'dynamicLiObj';
    	var html = '<li id=\'dynamicLiObj\' class=\'color6\'><span></span> : <a href=\'#\'></a>&nbsp&nbsp<button id=\'rmBtn6\' class=\"tooltipContent removeWordButton\" name="na" style="position: absolute; right: 0;">x</button></li>'
    	var template = document.createElement('template');
    	template.innerHTML = html;
    	var el = template.content.firstChild;
    	el.querySelectorAll('.removeWordButton').forEach(el => {
            //relatedRemoveButtons.push(el);
            el.setAttribute("name","defaultRemoval");
            el.addEventListener('click', ev => {
                ev.preventDefault();
                el.blur();
                removeWordButtonCallback(el);
            });
        });
    	return el;
    }

    let dynamicMainLegendDOMs = [];//to keep track of dynamically added entries

    function addSlotToMainLegend(){
    	/*add a new empty DOM li to main legend ul, the content is set in exploreword*/
    	var ul = document.getElementById("plotUL");
    	var el = assembleMainLegendDOM();
    	ul.append(el);
        dynamicMainLegendDOMs.push(el);
    	//force refresh main plot;
    	mainLegend = document.getElementById('mainLegend');
    	mainLegendItems = mainLegend.querySelectorAll('li');
    }

    function removeSlotFromMainLegend(){
    	/*in case other operations is needed in the future*/
    	mainLegendItems.pop();
    }

    function cleanMainLegend(){
    	/*remove all dynamically added slot from main legend*/
    	//console.log("cleanMainLegend ");
    	var numToIter = dynamicMainLegendDOMs.length;

    	for (let i = numToIter; i>0; i--)
    	{
    		var victim = dynamicMainLegendDOMs[i-1];
    		victim.parentNode.removeChild(victim);
    	}
    	//force refresh main plot;
    	mainLegend = document.getElementById('mainLegend');
    	mainLegendItems = mainLegend.querySelectorAll('li');
        //notify all dynamic objects are free now
        dynamicMainLegendDOMs.length = 0;
    }

    function exploreWord(word, mustIncludeList) {
    	var totalWordNum = 6;
    	//if (mustIncludeList != null&&mustIncludeList.length != 0)
    	if(mustIncludeListUpdated)
    	{
    		cleanMainLegend();
    		totalWordNum += mustIncludeList.length;
    		var currentLegendLength = mainLegendItems.length;
    		// this is to show how many slot are different
    		var slotNumDiff = mustIncludeList.length-(currentLegendLength-6);
    		
    		if (slotNumDiff>0)//we less entry slot in legend then needed
    		{
    			//console.log("adding legend items");
    			for (let i=slotNumDiff; i>0; i--)
    			{
    				addSlotToMainLegend();
    				//console.log(mainLegendItems.length);
    			}
    		}
    	}

        //console.log("function explorWord at index.js, must include word list is ", mustIncludeList);
        if (word !== currentWord||mustIncludeListUpdated == true) {
            currentWord = word;
            mustIncludeListUpdated = false;
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

                //other words contains the most interesting words, returned by handle
                let otherWords = handle.largest_changes_wrt(wordId, totalWordNum, 2, 2);
                console.log("other words before force replacement: ", otherWords);
                //replace last k interesting word to must included words
                if (mustIncludeList != null && mustIncludeList.length != 0)
                {
                	let i = totalWordNum;
                	let j = mustIncludeList.length;
                	for (let iter = j; iter>0; iter--)
                	{
                		otherWords.set([inverseVocab[mustIncludeWordList[j-1]]],i-1); 
                		i--;
                		j--;
                	} 
                }
                //console.log("MI list ",mustIncludeList);
                //to handle pair wise traj, a repetition is created; since handle.pairwise_trjectories must use array operation
                let wordIdRepeated = Array(totalWordNum).fill(wordId);
                let concatenatedTrajectories = handle.pairwise_trajectories(wordIdRepeated, otherWords);
                let trajectoryLength = concatenatedTrajectories.length / totalWordNum;
                //console.log("otherWords", otherWords);
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
                    //console.log("first elem child called");
                    const legendWordLabel = mainLegendItems[index].firstElementChild;
                    legendWordLabel.textContent = word;
                    //console.log("otherword ", otherWord);
                    legendWordLabel.nextElementSibling.textContent = otherWord;
                    if (legendWordLabel.nextElementSibling.nextElementSibling != null)
                    {
                        legendWordLabel.nextElementSibling.nextElementSibling.setAttribute("name",otherWord);
                    }
                });
                //console.log(mainLegendItems);
                mainLegend.style.visibility = 'visible';
            }
        }
    }

}())


/* TODO
function interpretQuery(string)
{
	return mainWO
}

function deployWithWordList(mainWord, mustIncluide)

*/