# Interne und externe Anbindung des Imports

<!-- source: https://amic.de/hilfe/_interneundexterneanb.htm -->

Die Import-Funktionalität steht selbstverständlich als JPP-Objekt zur Verfügung.

JPP-Objekt:

• JFA_Import

Methode:

• Free_Import

Parameter:

• fai_id die Ident(!) des Imports , zwingend erforderlich

• fai_pfad optionale Überschreibung des Import-Pfades

• receiver optionale Angabe Mail-Empfänger (notwendiger Spezialfall)

Damit ist es insbesondere skriptfähig und steht intern allen Anwender-Sprachen JPL, VBA und Makro zur Verfügung, extern findet es ohne weitere Verwendung über das A.eins-COM-Objekt.

<p class="siehe-auch">Siehe auch:</p>

- [Anwendungsbeispiel Outlook](./anwendungsbeispiel_outlook.md)
