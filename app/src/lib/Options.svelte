<script>
    import Output from './Output.svelte';
    import Input from './Input.svelte';

    let { getOutput, family, test, analysis, n, alpha, power, es, tail, allocRatio, k, p, q, m, rho, epsilon, nPredictors, df } = $props();

    const options = {
        t: [
            { id: null, text: "Correlation: Point biseral model", enabled: false, value: 1 },
            { id: null, text: "Linear bivariate regression: One group, size of slope", enabled: false, value: 2 },
            { id: null, text: "Linear bivariate regression: Two groups, difference between intercepts", enabled: false, value: 3 },
            { id: null, text: "Linear bivariate regression: Two groups, difference between slopes", enabled: false, value: 4 },
            { id: null, text: "Linear multiple regression: Fixed model, single regression coefficient", enabled: false, value: 5 },
            { id: "dependentSamplesTTest", text: "Means: Difference between two dependent means (matched pairs)", enabled: false, value: 6 },
            { id: "independentSamplesTTest", text: "Means: Difference between two independent means (two groups)", enabled: true, value: 7 },
            { id: "oneSampleTTest", text: "Means: Difference from constant (one sample case)", enabled: true, value: 8 },
            { id: null, text: "Means: Wilcoxon signed-rank test (matched pairs)", enabled: false, value: 9 },
            { id: null, text: "Means: Wilcoxon signed-rank test (one sample case)", enabled: false, value: 10 },
            { id: null, text: "Means: Wilcoxon-Mann-Whitney test (two groups)", enabled: false, value: 11 },
            { id: null, text: "Generic t test", enabled: false, value: 12 },
        ],
        f: [
            { id: "ANCOVA", text: "ANCOVA: Fixed effects, main effects, and interactions", enabled: true, value: 1 },
            { id: "oneWayANOVA", text: "ANOVA: Fixed effects, omnibus, one-way", enabled: true, value: 2 },
            { id: "twoWayANOVA", text: "ANOVA: Fixed effects, special, main effects, and interactions", enabled: true, value: 3 },
            { id: "betweenRepeatedANOVA", text: "ANOVA: Repeated measures, between factors", enabled: true, value: 4 },
            { id: "withinRepeatedANOVA", text: "ANOVA: Repeated measures, within factors", enabled: true, value: 5 },
            { id: "withinBetweenRepeatedANOVA", text: "ANOVA: Repeated measures, within-between interaction", enabled: true, value: 6 },
            { id: null, text: "Hotellings T²: One group mean vector", enabled: false, value: 7 },
            { id: null, text: "Hotellings T²: Two group mean vector", enabled: false, value: 8 },
            { id: null, text: "MANOVA: Global effects", enabled: false, value: 9 },
            { id: null, text: "MANOVA: Special effects and interactions", enabled: false, value: 10 },
            { id: null, text: "MANOVA: Repeated measures, between factors", enabled: false, value: 11 },
            { id: null, text: "MANOVA: Repeated measures, within factors", enabled: false, value: 12 },
            { id: null, text: "MANOVA: Repeated measures, within-between interaction", enabled: false, value: 13 },
            { id: "deviationFromZeroMultipleRegression", text: "Linear multiple regression: Fixed model, R² deviation from zero", enabled: true, value: 14 },
            { id: "increaseMultipleRegression", text: "Linear multiple regression: Fixed model, R² increase", enabled: true, value: 15 },
        ],
        chi: [
            { id: "goodnessOfFitChisqTest", text: "Goodness-of-fit tests: Contingency tables", enabled: true, value: 1 },
            { id: null, text: "Variance: Difference from constant (one sample case)", enabled: false, value: 2 },
            { id: null, text: "Generic χ² test", enabled: false, value: 3 },
        ],
    };

    // NOTE: Debug purposes only. Comment out when not needed.
    $inspect(family, test, analysis)
</script>

<div class="dropdowns border">

    <label for="family">
        Test family:<br>
        <select class="drop-down" id="family" bind:value={family} name="type">
            <option disabled value="exact">Exact</option>
            <option value="f">F tests</option>
            <option value="t">t tests</option>
            <option value="chi">χ² tests</option>
            <option disabled value="z">z tests</option>
        </select><br>
    </label>

    {#key family}
    <label for="test">
        Statistical test:<br>
        <select class="drop-down" id="test" bind:value={test} name="type">
            {#each options[family] as {id, text, value, enabled} (id ?? text)}
                <option value={id ?? text} disabled={!enabled}>{text}</option>
            {/each}
        </select><br>
    </label>
    {/key}

    <label for="analysis">
        Type of power analysis:<br>
        <select class="drop-down" id="analysis" bind:value={analysis} name="type">
            <option value="n">Compute required sample size (a priori)</option>
            <option value="alpha">Compute required α (criterion)</option>
            <option value="power">Compute achieved power (post hoc)</option>
            <option value="es">Compute required effect size (sensitivity)</option>
        </select>
    </label>

    <br>
    <br>
</div>

<div class="numbers">
    <Input family={family} test={test} analysis={analysis} bind:tail={tail} bind:allocRatio={allocRatio} bind:k={k} bind:p={p} bind:q={q} bind:m={m} bind:rho={rho} bind:epsilon={epsilon} bind:nPredictors={nPredictors} bind:df={df} />
    <Output getOutput={getOutput} family={family} test={test} analysis={analysis} bind:n={n} bind:alpha={alpha} bind:power={power} bind:es={es} tail={tail} allocRatio={allocRatio} k={k} p={p} q={q} m={m} rho={rho} epsilon={epsilon} nPredictors={nPredictors} df={df} />
</div>
