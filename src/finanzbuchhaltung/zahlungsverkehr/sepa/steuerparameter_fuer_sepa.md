# Steuerparameter für SEPA

<!-- source: https://amic.de/hilfe/steuerparameterfrsepa.htm -->

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen

Direktsprung **[SPA]**.

In der Steupagruppe „Optionen Finanzwesen“ existiert der Steuerparameter „DTA-Ausgabeformat“. Dieser muss auf **„SEPA“** eingestellt werden. Dieser Parameter sorgt unter anderem dafür, dass der Reorganisator zusätzliche Tests ausführt.

- Test der verwendeten Banken im Bankenstamm, ob der BIC eingetragen ist.
- Test der Kundenbanken, ob die IBAN eingetragen ist. Für Kundenbanken, deren Bank sich in Deutschland befindet und für die eine gültige Kontonummer bzw. eine gültige Bankleitzahl hinterlegt ist, wurde diese Nummer beim Update der Datenbank automatisch anhand dieser Daten generiert. Gleichzeitig wird getestet, ob die eingetragenen IBAN gültig ist. Dazu wird ein Prüfziffernverfahren angewandt, das auch bei der Stammdatenerfassung der IBAN eingesetzt wird.

  Der Test der IBAN kann entweder für jede [Bank](../stammdaten_zahlungsverkehr/bankenstamm.md) oder global per [Steuerparameter](../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md) abgeschaltet werden.

- Test der Hausbanken, ob die IBAN eingetragen worden ist. Die IBAN wird - wie auch bei den Kundenbanken – beim Update einmal für deutsche Banken mit eingetragener Kontonummer generiert. Bei eingetragenen IBANs wird auf Korrektheit nach dem Prüfziffernverfahren geprüft.

  Der Test der IBAN kann entweder für jede [Bank](../stammdaten_zahlungsverkehr/bankenstamm.md) oder global per [Steuerparameter](../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md) abgeschaltet werden.

Es sollte also vor der ersten Überweisung mit dem SEPA-Verfahren einmal der Test-Stammdaten durchgeführt werden.
