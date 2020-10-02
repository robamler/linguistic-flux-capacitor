import './default.css';

export function createPlot(
    containerElement, pointsX, ticksX, updateTooltipContents, tooltipTemplate, lineMouseover,
    lineMouseout
) {
    const BORDERS_MIN_X = 54;
    const BORDERS_MAX_X = 639;
    const BORDERS_MIN_Y = 1;
    const BORDERS_MAX_Y = 319;

    const MIN_X = BORDERS_MIN_X + 11;
    const MAX_X = BORDERS_MAX_X - 11;
    const MIN_Y = BORDERS_MIN_Y + 6;
    const MAX_Y = BORDERS_MAX_Y - 6;

    const SVG_WIDTH = 640;
    const SVG_HEIGHT = 360;

    let cursorTooltip = createTooltip(tooltipTemplate);
    let coordsX = [];
    let scaleX = null;
    let tenTimesMinYValue = null;
    let tenTimesMaxYValue = null;
    let scaleY = null;
    let offsetY = null;
    let plotPane = null;
    let xAxis = null;
    let yAxis = null;
    let lines = [];
    let mainLineIndex = null;
    let hideCursorTimeout = null;
    let hoverCursorContainer = null;
    let inputPrompt = null;
    let mousePrompt = null;
    let showMousePrompt = true;

    _initialize()
    return { plotLine, setMainLine, clear, hoverLine, unhoverLine, lineToFront };

    function _initialize() {
        const svg = createSvgElement('svg');
        svg.classList.add('plot');

        svg.setAttribute('width', SVG_WIDTH);
        svg.setAttribute('height', SVG_HEIGHT);
        svg.setAttribute('viewBox', '0 0 640 360');

        xAxis = svg.appendChild(
            createSvgElement('g', 'xAxis')
        );
        yAxis = svg.appendChild(
            createSvgElement('g', 'yAxis')
        );

        svg.appendChild(
            createSvgElement('rect', 'plotBorders', {
                x: BORDERS_MIN_X,
                y: BORDERS_MIN_Y,
                width: BORDERS_MAX_X - BORDERS_MIN_X,
                height: BORDERS_MAX_Y - BORDERS_MIN_Y,
            })
        );

        svg.appendChild(
            createSvgElement('text', 'axisLabel', {
                x: 0.5 * (BORDERS_MIN_X + BORDERS_MAX_X),
                y: SVG_HEIGHT - 5,
            })
        ).appendChild(
            document.createTextNode('year')
        );

        svg.appendChild(
            createSvgElement('text', 'axisLabel', {
                x: -0.5 * (BORDERS_MIN_Y + BORDERS_MAX_Y),
                y: 16,
                transform: 'rotate(-90)',
            })
        ).appendChild(
            document.createTextNode('word similarity')
        );

        let hoverAreasGroup = svg.appendChild(
            createSvgElement('g')
        );
        hoverAreasGroup.appendChild(
            createSvgElement('rect', 'invisible', {
                x: BORDERS_MIN_X,
                y: BORDERS_MIN_Y,
                width: BORDERS_MAX_X - BORDERS_MIN_X,
                height: BORDERS_MAX_Y - BORDERS_MIN_Y + 20,
                'data-index': '',
            })
        )

        scaleX = (MAX_X - MIN_X) / (pointsX[pointsX.length - 1] - pointsX[0]);
        for (let xPoint of pointsX) {
            coordsX.push(MIN_X + scaleX * (xPoint - pointsX[0]))
        }

        let ticksIndex = 0;
        for (let i = 0; i < coordsX.length; i += 1) {
            let mid_left = i === 0 ?
                BORDERS_MIN_X : 0.5 * (coordsX[i - 1] + coordsX[i]);
            let mid_right = i + 1 === coordsX.length ?
                BORDERS_MAX_X : 0.5 * (coordsX[i] + coordsX[i + 1]);
            hoverAreasGroup.appendChild(
                createSvgElement('rect', 'invisible', {
                    x: mid_left,
                    y: BORDERS_MIN_Y,
                    width: mid_right - mid_left,
                    height: BORDERS_MAX_Y - BORDERS_MIN_Y + 20,
                    'data-index': i,
                })
            );

            if (pointsX[i] == ticksX[ticksIndex]) {
                xAxis.appendChild(
                    createSvgElement('path', 'axisTick', {
                        d: 'M ' + coordsX[i] + ',' + BORDERS_MAX_Y + ' v -5',
                    })
                );
                xAxis.appendChild(
                    createSvgElement('path', 'axisTick', {
                        d: 'M ' + coordsX[i] + ',' + BORDERS_MIN_Y + ' v 5',
                    })
                );

                xAxis.appendChild(
                    createSvgElement('text', 'axisTickLabel', {
                        x: coordsX[i],
                        y: BORDERS_MAX_Y + 15,
                    })
                ).appendChild(
                    document.createTextNode('' + ticksX[ticksIndex])
                );

                ticksIndex += 1;
            }
        }

        plotPane = svg.appendChild(
            createSvgElement('g')
        );

        hoverCursorContainer = svg.appendChild(
            createSvgElement('g', ['hoverCursorContainer', 'hidden'])
        );

        inputPrompt = createSvgElement('text', 'plotPrompt', { y: 80 });
        ['↑', 'enter a word into', 'the above field.'].forEach((text, index) => {
            let tspan = createSvgElement('tspan', null, { x: 347, dy: ['0', '1.2em', '1.1em'][index] });
            tspan.appendChild(document.createTextNode(text));
            inputPrompt.appendChild(tspan);
        });
        svg.appendChild(inputPrompt);

        mousePrompt = createSvgElement('g', 'plotPrompt');
        for (let i = 0; i < 2; i += 1) {
            let current = createSvgElement('text', i === 0 ? 'plotPromptOutline' : '', { y: 95 });
            ['Move mouse', 'across this area', 'to explore more.'].forEach((text, index) => {
                let tspan = createSvgElement('tspan', null, { x: 347, dy: index === 0 ? '0' : '1.2em' });
                tspan.appendChild(document.createTextNode(text));
                current.appendChild(tspan);
            });
            mousePrompt.appendChild(current);
        }
        mousePrompt.addEventListener('mouseover', () => {
            mousePrompt.style.opacity = 0;
            showMousePrompt = false;
            setTimeout(() => mousePrompt.style.display = 'none', 500);
        });

        svg.appendChild(mousePrompt);

        // Add invisible area just above or below cursor to prevent mouseover
        // events when the user moves from the cursor to the tooltip.
        hoverCursorContainer.appendChild(
            createSvgElement('rect', ['invisible', 'noCaptureZone'], {
                x: -40,
                y: -45,
                width: 80,
                height: 47,
            })
        );

        let hoverCursor = hoverCursorContainer.appendChild(
            createSvgElement('circle', 'hoverCursor', {
                cx: 0,
                cy: 0,
                r: 8,
                'data-index': '',
            })
        );

        let hoverColorIndex = null;
        let hoverOver = event => {
            if (mainLineIndex !== null) {
                clearTimeout(hideCursorTimeout);
                let index = event.target.getAttribute('data-index');
                let x = coordsX[index];
                if (typeof x !== 'undefined') {
                    let line = lines[mainLineIndex];
                    let y = line.coordsY[index];
                    let svgBox = svg.getBoundingClientRect();
                    let svgTop = svgBox.top;
                    let svgBottom = svgBox.bottom;
                    let mouseY = ((event.clientY - svgTop) / (svgBottom - svgTop)) * SVG_HEIGHT;
                    hoverCursorContainer.setAttribute('transform', 'translate(' + x + ',' + y + ')');

                    if (line.colorIndex !== hoverColorIndex) {
                        hoverCursor.classList.remove('color' + hoverColorIndex);
                        hoverColorIndex = line.colorIndex;
                        hoverCursor.classList.add('color' + hoverColorIndex);
                    }
                    hoverCursorContainer.classList.remove('hidden');

                    showMousePrompt = false;
                    mousePrompt.style.opacity = 0;
                    setTimeout(() => mousePrompt.style.display = 'none', 500);
                    updateTooltipContents(cursorTooltip, line, index);
                    showTooltip(cursorTooltip, line, index, mouseY < y);
                }
            }
        };

        hoverCursorContainer.addEventListener('mouseover', hoverOver);
        cursorTooltip.querySelector('.tooltipMain').addEventListener('mouseover', hoverOver);
        cursorTooltip.querySelector('.tooltipPointer').addEventListener('mouseover', hoverOver);
        hoverAreasGroup.addEventListener('mouseover', hoverOver);

        hoverCursorContainer.addEventListener('mouseout', hoverOut);
        cursorTooltip.querySelector('.tooltipMain').addEventListener('mouseout', hoverOut);
        cursorTooltip.querySelector('.tooltipPointer').addEventListener('mouseout', hoverOut);
        hoverAreasGroup.addEventListener('mouseout', hoverOut);

        // containerElement.style.position = 'relative';
        containerElement.appendChild(cursorTooltip);
        containerElement.appendChild(svg);
    }

    function plotLine(valuesY, colorIndex, styleIndex, payload, isMainLine, title) {
        isMainLine = !!isMainLine || lines.length === 0;

        inputPrompt.style.opacity = 0;
        setTimeout(() => {
            if (inputPrompt.style.opacity == 0) { // Yes, we want == and not === here.
                inputPrompt.style.display = 'none';
            }
        }, 500);
        if (showMousePrompt) {
            mousePrompt.style.display = 'block';
            mousePrompt.style.opacity = 0.7;
        }

        const cur10MinYValue = 10 * Math.min(...valuesY);
        const cur10MaxYValue = 10 * Math.max(...valuesY);

        hideTooltip();

        if (scaleY === null
            || cur10MinYValue < tenTimesMinYValue
            || cur10MaxYValue > tenTimesMaxYValue
        ) {
            tenTimesMinYValue = tenTimesMinYValue === null ?
                cur10MinYValue : Math.min(tenTimesMinYValue, cur10MinYValue);
            tenTimesMaxYValue = tenTimesMaxYValue === null ?
                cur10MaxYValue : Math.max(tenTimesMaxYValue, cur10MaxYValue);

            let displayRangeTop = Math.min(
                10, 1.02 * tenTimesMaxYValue - 0.02 * tenTimesMinYValue);
            let displayRangeBottom = Math.max(
                -10, 1.02 * tenTimesMinYValue - 0.02 * tenTimesMaxYValue);

            // Make sure that there are at least 2 viable y-axis ticks (at one decimal precision).
            if (Math.floor(displayRangeTop) - Math.ceil(displayRangeBottom) <= 0) {
                displayRangeBottom = Math.ceil(displayRangeBottom - 1);
                displayRangeTop = Math.floor(displayRangeTop + 1);
            }
            if (displayRangeTop > 10) {
                displayRangeBottom -= displayRangeTop - 10;
                displayRangeTop = 10;
            } else if (displayRangeBottom < -10) {
                displayRangeTop -= displayRangeBottom + 10;
                displayRangeBottom = 10;
            }

            scaleY = 10 * (MIN_Y - MAX_Y) / (displayRangeTop - displayRangeBottom);
            offsetY = MIN_Y - 0.1 * displayRangeTop * scaleY;

            // Update y-axis ticks.
            while (yAxis.childNodes.length !== 0) {
                // For some reason, this has to be called in an extra loop or else it won't
                // remove all child nodes.
                yAxis.childNodes.forEach(child => yAxis.removeChild(child));
            }
            let multiples = [1, 2, 5];
            let lowestYTick = Math.ceil(displayRangeBottom);
            while ((displayRangeTop - lowestYTick) / multiples[0] >= 7 && multiples.length !== 1) {
                multiples.shift();
                lowestYTick += multiples[0] - 1 - (19 + lowestYTick) % multiples[0];
            }
            if (lowestYTick <= 0 && displayRangeTop >= 0) {
                // Draw zero line
                yAxis.appendChild(
                    createSvgElement('path', 'zeroLine', {
                        d: 'M ' + BORDERS_MIN_X + ',' + offsetY + ' h ' + (BORDERS_MAX_X - BORDERS_MIN_X)
                    })
                );
            }
            for (let pos = lowestYTick; pos <= displayRangeTop; pos += multiples[0]) {
                let tickY = 0.1 * pos * scaleY + offsetY;
                yAxis.appendChild(
                    createSvgElement('path', 'axisTick', {
                        d: 'M ' + BORDERS_MIN_X + ',' + tickY + ' h 5',
                    })
                );
                yAxis.appendChild(
                    createSvgElement('path', 'axisTick', {
                        d: 'M ' + BORDERS_MAX_X + ',' + tickY + ' h -5',
                    })
                );

                yAxis.appendChild(
                    createSvgElement('text', 'axisTickLabel', {
                        x: BORDERS_MIN_X - 3,
                        y: tickY + 5.65236,
                    })
                ).appendChild(
                    document.createTextNode((0.1 * pos).toFixed(1))
                );
            }

            // Rescale all existing lines.
            for (let line of lines) {
                // TODO: factor out code duplication (see below).
                line.coordsY = line.valuesY.map(value => value * scaleY + offsetY);
                const pathD = 'M ' + coordsX.map(
                    (x, i) => x + ',' + line.coordsY[i]).join(' ');
                line.lineGroup.childNodes.forEach(
                    child => child.setAttribute('d', pathD)
                );
            }
        }

        const coordsY = valuesY.map(value => value * scaleY + offsetY);
        const pathD = 'M ' + coordsX.map((x, i) => x + ',' + coordsY[i]).join(' ');

        let classNames = ['lineGroup', 'color' + colorIndex];
        const lineGroup = createSvgElement('g', classNames);

        plotPane.insertBefore(lineGroup, plotPane.firstChild);

        lineGroup.appendChild(
            createSvgElement('path', 'bleed', {
                d: pathD
            })
        );
        lineGroup.appendChild(
            createSvgElement('path', 'line', {
                d: pathD
            })
        );

        let mouseCapture = createSvgElement('path', 'mouseCapture', {
            d: pathD,
        });
        let hoverTitle = createSvgElement('title');
        hoverTitle.appendChild(document.createTextNode(title));
        mouseCapture.appendChild(hoverTitle)
        lineGroup.appendChild(mouseCapture);

        let line = {
            valuesY,
            coordsY,
            colorIndex,
            styleIndex,
            lineGroup,
            payload,
        };

        lineGroup.addEventListener('click', () => setMainLine(lines.indexOf(line)));
        lineGroup.addEventListener('mouseover', () => lineMouseover(lines.indexOf(line)));
        lineGroup.addEventListener('mouseout', () => lineMouseout(lines.indexOf(line)));
        lines.push(line);

        if (isMainLine) {
            setMainLine(lines.length - 1);
        }
    }

    function hoverLine(index) {
        if (mainLineIndex !== null) {
            lines[mainLineIndex].lineGroup.classList.remove('main');
        }
        let selectedLine = lines[index];
        if (typeof selectedLine !== 'undefined') {
            selectedLine.lineGroup.classList.add('hovering');
        }
    }

    function unhoverLine(index) {
        let selectedLine = lines[index];
        if (typeof selectedLine !== 'undefined') {
            selectedLine.lineGroup.classList.remove('hovering');
        }
        if (mainLineIndex !== null) {
            lines[mainLineIndex].lineGroup.classList.add('main');
        }
    }

    function clear() {
        hideTooltip();
        tenTimesMinYValue = null;
        tenTimesMaxYValue = null;
        scaleY = null;
        offsetY = null;
        lines = [];
        mainLineIndex = null;

        while (yAxis.childNodes.length !== 0) {
            // For some reason, this has to be called in an extra loop or else it won't
            // remove all child nodes.
            yAxis.childNodes.forEach(child => yAxis.removeChild(child));
        }
        while (plotPane.childNodes.length != 0) {
            plotPane.childNodes.forEach(el => el.remove());
        }

        mousePrompt.style.opacity = 0;
        setTimeout(() => {
            if (mousePrompt.style.opacity == 0) { // Yes, we want == and not === here.
                mousePrompt.style.display = 'none';
            }
        }, 500);
        inputPrompt.style.display = 'block';
        inputPrompt.style.opacity = 0.7;
    }

    function hideTooltip() {
        hoverCursorContainer.classList.add('hidden');
        cursorTooltip.classList.add('hidden');
        // Set `display: none` on tooltip once it's faded out so that it doesn't users
        // can access page elements underneath it.
        setTimeout(() => cursorTooltip.classList.add('undisplayed'), 300);
    }

    function hoverOut() {
        hideCursorTimeout = setTimeout(hideTooltip, 400);
    }

    function setMainLine(lineIndex) {
        let selectedLine = lines[lineIndex];
        if (typeof selectedLine !== 'undefined') {
            lineMouseout(lineIndex);

            let previousMainLine = lines[mainLineIndex];
            if (typeof previousMainLine !== 'undefined') {
                previousMainLine.lineGroup.classList.remove('main');
            }
            mainLineIndex = lineIndex;

            selectedLine.lineGroup.classList.add('main');
            lineToFront(lineIndex);
        }
    }

    function lineToFront(lineIndex) {
        if (typeof lineIndex === 'undefined') {
            lineIndex = mainLineIndex;
        }
        let selectedLine = lines[lineIndex];
        if (typeof selectedLine !== 'undefined') {
            const lineGroup = selectedLine.lineGroup;
            lineGroup.remove();
            plotPane.appendChild(lineGroup)
        }
    }

    function showTooltip(tooltip, line, index, showBelow) {
        let oldColorIndex = tooltip.getAttribute('data-color-index');
        if ((oldColorIndex != line.colorIndex)) {
            tooltip.classList.remove('color' + oldColorIndex);
            tooltip.classList.add('color' + line.colorIndex);
            tooltip.setAttribute('data-color-index', line.colorIndex);
        }

        let coordY = line.coordsY[index];
        let x = coordsX[index] * containerElement.offsetWidth / SVG_WIDTH;
        let y = coordY * containerElement.offsetHeight / SVG_HEIGHT;
        tooltip.style.left = x + 'px';
        tooltip.style.top = y + 'px';

        if (showBelow) {
            tooltip.classList.add('pointsUp');
            hoverCursorContainer.classList.add('upsideDown');
        } else {
            tooltip.classList.remove('pointsUp');
            hoverCursorContainer.classList.remove('upsideDown');
        }

        tooltip.classList.remove('hidden');
        tooltip.classList.remove('undisplayed');
    }
}

function createSvgElement(tagName, classNames, attributes) {
    const element = document.createElementNS('http://www.w3.org/2000/svg', tagName);

    if (classNames) {
        if (typeof classNames === 'string') {
            element.classList.add(classNames);
        } else {
            for (let c of classNames) {
                element.classList.add(c);
            }
        }
    }

    if (attributes) {
        for (var key in attributes) {
            if (attributes.hasOwnProperty(key)) {
                element.setAttribute(key, '' + attributes[key]);
            }
        }
    }

    return element;
}

function createTooltip(template) {
    const tooltip = document.createElement('div');
    tooltip.setAttribute('data-color-index', '0');
    tooltip.classList.add('plot');
    tooltip.classList.add('tooltip');
    tooltip.classList.add('hidden');
    tooltip.classList.add('undisplayed');
    tooltip.classList.add('color0');
    tooltip.appendChild(template);

    return tooltip;
}
