import './default.css';

export function createPlot(
    containerElement, pointsX, ticksX, updateTooltipContents, tooltipOrientationThreshold
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

    if (typeof tooltipOrientationThreshold == 'undefined') {
        tooltipOrientationThreshold = 0.5;
    }

    _initialize()
    return { plotLine, setMainLine };

    function _initialize() {
        const cursorTooltip = createTooltip();

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
            document.createTextNode('cosine similarity')
        );

        let hoverAreasGroup = svg.appendChild(
            createSvgElement('g')
        );
        hoverAreasGroup.appendChild(
            createSvgElement('rect', 'invisible', {
                x: BORDERS_MIN_X,
                y: BORDERS_MIN_Y,
                width: BORDERS_MAX_X - BORDERS_MIN_X,
                height: BORDERS_MAX_Y - BORDERS_MIN_Y,
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
                    height: BORDERS_MAX_Y - BORDERS_MIN_Y,
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

        // Add invisible area just above or below cursor to prevent mouseover
        // events when the user moves from the cursor to the tooltip.
        // TODO: set hoverCursorContainer.upsideDown CSS class when tooltip is below
        hoverCursorContainer.appendChild(
            createSvgElement('rect', ['invisible', 'noCaptureZone'], {
                x: -8.75,
                y: -30,
                width: 17.5,
                height: 30,
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
                    hoverCursorContainer.setAttribute('transform', 'translate(' + x + ',' + y + ')');

                    if (line.colorIndex !== hoverColorIndex) {
                        hoverCursor.classList.remove('color' + hoverColorIndex);
                        hoverColorIndex = line.colorIndex;
                        hoverCursor.classList.add('color' + hoverColorIndex);
                    }
                    hoverCursorContainer.classList.remove('hidden');

                    updateTooltipContents(cursorTooltip, line, index);
                    showTooltip(cursorTooltip, line, index);
                }
            }
        };

        let hoverOut = event => {
            hideCursorTimeout = setTimeout(() => {
                hoverCursorContainer.classList.add('hidden');
                cursorTooltip.classList.add('hidden');
                if (typeof tooltipHandle !== 'undefined') {
                    hideTooltip();
                }
            }, 400);
        };

        hoverCursorContainer.addEventListener('mouseover', hoverOver);
        cursorTooltip.querySelector('.tooltipMain').addEventListener('mouseover', hoverOver);
        cursorTooltip.querySelector('.tooltipPointer').addEventListener('mouseover', hoverOver);
        hoverAreasGroup.addEventListener('mouseover', hoverOver);

        hoverCursorContainer.addEventListener('mouseout', hoverOut);
        cursorTooltip.querySelector('.tooltipMain').addEventListener('mouseout', hoverOut);
        cursorTooltip.querySelector('.tooltipPointer').addEventListener('mouseout', hoverOut);
        hoverAreasGroup.addEventListener('mouseout', hoverOut);

        containerElement.style.position = 'relative';
        containerElement.appendChild(cursorTooltip);
        containerElement.appendChild(svg);
    }

    function plotLine(valuesY, colorIndex, styleIndex, payload, isMainLine) {
        isMainLine = !!isMainLine || lines.length === 0;
        const cur10MinYValue = 10 * Math.min(...valuesY);
        const cur10MaxYValue = 10 * Math.max(...valuesY);

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
            // while (Math.floor(scaledMaxYValue) - Math.ceil(scaledMinYValue) < 2) {
            //     let roomBelow = scaledMinYValue - Math.floor(scaledMinYValue);
            //     let roomAbove = Math.ceil(scaledMaxYValue) - scaledMaxYValue;
            //     if (roomAbove > roomBelow) {
            //         scaledMinYValue = Math.ceil(scaledMinYValue - 1);
            //     } else {
            //         scaledMaxYValue = Math.floor(scaledMaxYValue + 1);
            //     }
            // }
            if (displayRangeTop > 10) {
                displayRangeBottom -= displayRangeTop - 10;
                displayRangeTop = 10;
            } else if (displayRangeBottom < -10) {
                displayRangeTop -= displayRangeBottom + 10;
                displayRangeBottom = 10;
            }

            scaleY = 10 * (MIN_Y - MAX_Y) / (displayRangeTop - displayRangeBottom);
            offsetY = MAX_Y + 0.1 * displayRangeTop * scaleY;

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
        lineGroup.appendChild(
            createSvgElement('path', 'mouseCapture', {
                d: pathD
            })
        );

        let line = {
            valuesY,
            coordsY,
            colorIndex,
            styleIndex,
            lineGroup,
            payload,
        };

        lineGroup.addEventListener('click', () => setMainLine(lines.indexOf(line)));
        lines.push(line);

        if (isMainLine) {
            setMainLine(lines.length - 1);
        }
    }

    function setMainLine(lineIndex) {
        for (let otherLine of lines) {
            otherLine.lineGroup.classList.remove('main');
        }

        const lineGroup = lines[lineIndex].lineGroup;
        lineGroup.classList.add('main');
        lineGroup.remove();
        plotPane.appendChild(lineGroup)

        mainLineIndex = lineIndex;
    }

    function showTooltip(tooltip, line, index) {
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

        let wasBelow = tooltip.classList.contains('pointsUp');
        let relativeY = (coordY - MIN_Y) / (MAX_Y - MIN_Y);
        console.log(relativeY);

        let tooltipBelow = relativeY < tooltipOrientationThreshold - 0.05 || (
            wasBelow && relativeY <= tooltipOrientationThreshold + 0.05);
        if (tooltipBelow != wasBelow) {
            tooltip.classList.toggle('pointsUp');
            hoverCursorContainer.classList.toggle('upsideDown');
        }

        tooltip.classList.remove('hidden');
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

function createTooltip() {
    const tooltip = document.createElement('div');
    tooltip.setAttribute('data-color-index', '0');
    tooltip.classList.add('plot');
    tooltip.classList.add('tooltip');
    tooltip.classList.add('hidden');
    tooltip.classList.add('color0');
    tooltip.innerHTML = "<div class='tooltipInnerContainer'><div class='tooltipMain'><div class='tooltipContent'><div class='year'>1890</div><div class='mainWord'>word</div>was most related to:<div class='wait'>(crunching numbers,<br>please stand by&nbsp;...)</div><ul class='suggestions'><li class='suggestion'></li><li class='suggestion'></li><li class='suggestion'></li><li class='suggestion'></li><li class='suggestion'></li><li class='suggestion'></li><li class='suggestion'></li></ul></div></div><div class='tooltipPointer'></div></div>";

    return tooltip;
}
