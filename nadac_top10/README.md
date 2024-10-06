# Rust Interview Question

The National Average Drug Acquisition Cost (NADAC) comparison dataset tracks per unit drug price changes. Please write a short rust program to process [nadac-comparison-04-17-2024.csv][1] and report on the 10 largest per unit price increases and 10 largest per unit price decreases with effective dates in 2023. Duplicate description, price change entries with an exact price change match should be eliminated. The program should be implemented in async rust using tokio, reqwest, and csv crates. The implementation should be memory efficient and perform well.

Report output based on rows with an effective date in 2020 is provided below. Your program output should match this format. Write a unit test to prove your code can reproduce this 2020 report.

More information about the NADAC Comparison dataset can be found here: [NADAC Comparison][2] and in appendix 6 here: [nadacmethodology.pdf][3].

# Implementation Checklist

- ⏹️ Implement your program in async Rust using reqwest, tokio, and csv crates
- ⏹️ Do not use AI code generation
- ⏹️ Consider only data rows with an **Effective Date** in the given year
- ⏹️ Calculate the monetary **NADAC Per Unit** price change per row from the new and old values
- ⏹️ Eliminate duplicate **NDC Description**, NADAC Per Unit price change entries
- ⏹️ Only duplicates with an exact per unit price change should be eliminated
- ⏹️ Report the top 10 NADAC Per Unit price change increases and descreases
- ⏹️ The full precision price change ordering should be used to identify the top results
- ⏹️ Ensure that the solution is as memory efficient as possible
- ⏹️ Storing a full years worth of data or more in memory is not memory efficient
- ⏹️ Confirm that the solution performs well

# 2020 Top 10 Report

```
Top 10 NADAC per unit price increases of 2020:
$1054.18: STELARA 45 MG/0.5 ML SYRINGE
$1048.40: STELARA 90 MG/ML SYRINGE
$420.33: SIMPONI 50 MG/0.5 ML PEN INJEC
$333.98: LUPRON DEPOT 22.5 MG 3MO KIT
$292.70: CIMZIA 2X200 MG/ML SYRINGE KIT
$277.98: LUPRON DEPOT 11.25 MG 3MO KIT
$186.57: HUMIRA PEN 40 MG/0.8 ML
$186.52: HUMIRA(CF) 40 MG/0.4 ML SYRINGE
$186.50: HUMIRA(CF) PEN 40 MG/0.4 ML
$186.47: HUMIRA 40 MG/0.8 ML SYRINGE

Top 10 NADAC per unit price decreases of 2020:
-$117.40: DIHYDROERGOTAMINE MESYLATE 4 MG/ML NASAL SPRAY
-$100.02: STELARA 90 MG/ML SYRINGE
-$71.51: STELARA 90 MG/ML SYRINGE
-$66.48: STELARA 90 MG/ML SYRINGE
-$31.57: ALBENDAZOLE 200 MG TABLET
-$30.56: CINACALCET HCL 90 MG TABLET
-$27.71: ACYCLOVIR 5% CREAM
-$26.74: SUMATRIPTAN 4 MG/0.5 ML INJECT
-$26.13: DIHYDROERGOTAMINE MESYLATE 4 MG/ML NASAL SPRAY
-$23.85: ALBENDAZOLE 200 MG TABLET
```

[1]: https://download.medicaid.gov/data/nadac-comparison-04-17-2024.csv
[2]: https://data.medicaid.gov/dataset/a217613c-12bc-5137-8b3a-ada0e4dad1ff
[3]: https://www.medicaid.gov/medicaid-chip-program-information/by-topics/prescription-drugs/ful-nadac-downloads/nadacmethodology.pdf
