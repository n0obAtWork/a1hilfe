# Allgemeiner Steuerparameter für die Vorgangsimportschnittstelle (SPA 928)

<!-- source: https://amic.de/hilfe/_SPA_928.htm -->

An diesem Steuerparameter werden Optionen für die Vorgangsimportschnittstelle eingetragen.

#### Folgende Optionen sind verfügbar:

| Option | Wert |
| --- | --- |
| UNTERKLASSERECHNUNG | Hier kann eine abweichende Unterklasse für die Rechnung hinterlegt werden. Die Unterklasse wird bei der Belegerzeugung ausgewertet und überschreibt die Unterklasse aus der Relation ImportVorgStamm. Ist in diesem Steuerparameter diese Option nicht hinterlegt, so wird die Unterklasse aus der Relation ImportVorgStamm genommen. |
| UNTERKLASSEGUTSCHRIFT | Hier kann eine abweichende Unterklasse für die Gutschrift hinterlegt werden. Die Unterklasse wird bei der Belegerzeugung ausgewertet und überschreibt die Unterklasse aus der Relation ImportVorgStamm. Ist in diesem Steuerparameter diese Option nicht hinterlegt, so wird die Unterklasse aus der Relation ImportVorgStamm genommen. |
| UNTERKLASSEUNBEKANNTERBELEG | Hier kann eine Unterklasse für einen undefinierten Vorgang hinterlegt werden. Beim Einspielen der Daten in die Relation ImportVorgStamm muss der Steuerparameter ausgewertet werden. Die Unterklasse muss dann mit dem Wert aus dem Steuerparameter gepflegt werden. Diese Option wird vom Standard nicht ausgewertet. |
| KLASSEUNBEKANTERBELEG | Hier kann eine Klasse für einen undefinierten Vorgang hinterlegt werden. Beim Einspielen der Daten in die Relation ImportVorgStamm muss der Steuerparameter ausgewertet werden. Die Klasse muss dann mit dem Wert aus dem Steuerparameter gepflegt werden. . Diese Option wird vom Standard nicht ausgewertet. |
| UMSCHLUESSELKUNDEDATEIIMPORT | Hier kann eine Umschlüsselungsklasse für den Kunden hinterlegt werden. Diese Option wird vom Standard nicht bedient. |
| BESTELLVORSCHLAEGELAGERTRENNUNG | Mit diesem Steuerparameter kann eingestellt werden, ob bei der Erzeugung von Bestellungen aus Bestellvorschlägen die Bestellungen nach Lägern getrennt werden sollen.<br>0 - Es findet keine Trennung nach Lägern statt<br>1 - Es findet eine Trennung nach Lägern statt<br>Ist dieser Steuerparameter nicht gesetzt, so findet keine Trennung nach Lägern statt. |
| GLOBALEAENDERUNGSPROZEDUR | Hier kann eine private Prozedur / Funktion hinterlegt werden, mit dieser können die Daten im Vorgangsimport vor der Erzeugung eines Vorgangs noch verändert werden. |
