# Makro-Pfleger

<!-- source: https://amic.de/hilfe/_makro_pfleger.htm -->

Der Makro-Pfleger ermöglicht Ansehen, Bearbeiten, Löschen und Neuanlage eines Makro-Programmes.

Über „F8“ neu-angelegte Makro-Programme erhalten automatisch:

1) den Besitzer Privat“

2) wird ihnen ein syntaktisch richtiger Makro-Text zugeordnet. Dieser Makro-Text kann dann weiterverarbeitet werden.

| Felder | |
| --- | --- |
| Makroname | „Bezeichner“ des Makros. Über diesen Identifier werden Makros gestartet. |
| Typ | |
| ID | Informatorisch die interne Identifikations-ID (diese ID ist mit dem Besitzer systemweit eindeutig) |
| Parameter 1 | Im Pfleger vorgebbare Parameter |
| Parameter 2 | |
| Parameter 3 | |
| Parameter 4 | |
| Resultat | Optionale Ergebnis-Rückgabe des Makro-Programmes |
| Debugger | Status eines ggf. verbundenen „Debugger“ (Extra-Software zum Überprüfen von Laufzeitverhalten) |
| Vorgang, Zähler, Datensatz | Weitere optionale Ergebnis-Rückgaben des Makro-Programmes. |
| Scriptausgaben | Weitere optionale Laufzeit-Mitteilungen des Makro-Programmes. |

Neben den Pflegerfunktionen „Speichern“, „Neu“ usw. sind noch folgende Funktion möglich:

| Funktionen | |
| --- | --- |
| **aus Datei laden** | Lädt aus einer angebbaren \*.pas-Datei den ASCII-Text als Makro-Programm-Text. |
| **übersetzen** | Kompiliert den zugeordneten Makro-Programm-Text in eine durch A.eins ausführbare Informationseinheit. |
| **Script vergleichen** | |
| **Version speichern** | |
| **Version wiederherstellen** | |
| **Editor aufrufen** | Ruft den einstellbaren Editor zum Bearbeiten des Makro-Programm-Textes auf. |
| **ausführen** | Führt das Makro-Programm mit den unter Parameter 1-4 angegebenen Parametern aus dem Pfleger heraus aus. |
| **Debugger umschalten** | |
| **Debugger aktualisieren** | |
| **Fehlermeldungen** | Die Funktion „übersetzen“ kann feststellen das es syntaktische Probleme mit dem Makro-Programm-Text gibt. Mit dieser Funktion werden detaillierte Hinweise abgerufen um die Möglichkeit zu bekommen, die Probleme zu beheben. |
| **SQL-Auslagerung** | Schreibt ein Sql-Skript in eine vorgebbare Sql-Datei. Diese kann z.B. zur Einspielung in A.eins-Datenbanken verwendet werden. |
| **hole von IMP++** | Methode um IMP++-Scripte in Makro-Programm-Texte zu überführen (diese Methode dürfte obsolet sein) |
| **Script-Parameter** | Ruft die Anwendung „Script-Parameter“ auf. |
| **in Datei sichern** | Schreibt den Makro-Programm-Text in eine vorgebbare \*.pas – Datei. |
