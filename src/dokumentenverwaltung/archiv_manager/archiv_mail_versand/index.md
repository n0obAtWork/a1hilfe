# Archiv Mail Versand

<!-- source: https://amic.de/hilfe/_archivmailversand.htm -->

Mit der Funktion „**Senden An**“ ![](../../../ImagesExt/image8_893.png) in der Archivanzeige können Sie ein oder mehrere markierte Dokumente per Mail und/oder Fax versenden.

Diese Funktion steht u.a. auch in den Auswahllisten Kunden, Anschriften, Lieferanten und Interessenten als Funktion „**Email senden**“ zur Verfügung,

Einem Dokument ist in der Regel ein Kunde zugeordnet, der jedoch neben der Hauptanschrift noch ein Lager und mehrere Ansprechpartner haben kann. Es wird also eine Liste der möglichen E-Mail-Adressen und Faxnummern angezeigt, an die die gewählten Dokumente versendet werden sollen. Zudem gibt es ein Eingabefeld, in dem Sie manuell eine oder mehrere Mailadressen bzw. Faxnummern mit Semikolon getrennt eingeben können.

Ein Betreff und ein kurzer Beschreibungstext können hier ebenso für die Verwendung in einer E-Mail oder einem Faxdeckblatt angegeben werden.

Nun stehen folgende Funktionen zur Verfügung:

• Versenden – Hier wird die Liste der Dokumente und Empfänger an ein externes System übermittelt, das sodann den Versand übernimmt. Hier kann z.B. Tobit verwendet werden. Als Schnittstelle dient ein XML-Dokument, das die für den Versand notwendigen Daten bereitstellt.

• Versenden (Outlook) – Hier wird das im [Einrichterparameter](../../../firmenstamm/einrichterparameter/archiv_mail_versand_epa_fa_mail.md) gesetzte Skript aufgerufen, das die gewählten Dokumente aus der Datenbank extrahiert und ein installiertes Outlook dazu veranlasst, eine Mail zu öffnen, die die gewählten Empfänger und die gewählten Dokumente beinhaltet. Die Mail muss dann manuell abgesendet werden.

Es ist möglich mit einem alternativen VBS-Skript an dieser Stelle andere Mailclients anzusprechen.

Für die Anbindung eines Tobit-Clients steht exemplarisch das VBA-Skript „AMIC_FAVersandTobit“ zur Verfügung.

Neben der ersten Registerkarte, auf der diese Einstellungen vorgenommen werden können, stehen u.U. weitere Registerkarten mit einem Datum im Tab-Reiter. Diese geben dem Anwender die Möglichkeit als eine Historie nachzuvollziehen, wann das Dokument an wen versendet wurde.

Die zu versendenden Anlagen werden nun in einem Grid informatorisch aufgelistet. Angezeigt werden hierzu jeweils die Belegklasse, sowie der Beleg-Typ. Mit der Funktion Anlagen hinzufügen lassen sich über die - im Bedarfsfalle privatisierbare - Itembox IB_FA weitere Anlagen aus dem Archiv hinzufügen. Die Funktion Archiv anzeigen öffnet die Dokumente des Archivs zur Ansicht.

Mit Hilfe der Funktion Senden festlegen… können Sie einen Assistenten aktivieren, der hilft, bei Empfängern die Senden-Einstellung zu beeinflussen. Es kann damit die Einstellung aller Mail – bzw. Fax-Adressen bzgl. des Sendeverhaltens beeinflusst werden. Anwendungsbeispiel ist die Funktion E-Mail senden aus dem Anschriftstamm heraus. Dort hat man eine Reihe von Empfängern vorselektiert und das System sucht Mail – und Fax-Adressaten heraus. Über Senden festlegen… kann man nun alle Mail-Adressen „aktivieren“.

Bei der Zusammenstellung der E-Mail-Adressen werden die folgenden Punkte herangezogen:

\- Zugehöriger (Ober-/Rechnungs-)Kunden mit all seinen Anschriften

\- Im EPA individuell eingerichtete Spezialadressen

\- Anpassen kann man die Zusammenstellung der Anschriften in der Prozedur *„FA_mail_senden_an_Adressen“*

Über eine private Prozedur kann die Versorgung der angezeigten E-Mail-Adressen privatisiert werden (Einrichterparameter).

Über einen [Einrichterparameter](../../../firmenstamm/einrichterparameter/archiv_mail_versand_epa_fa_mail.md) kann festgelegt werden, ob Anlagen standardmäßig mitgeschickt werden sollen oder nicht.

<p class="siehe-auch">Siehe auch:</p>

- [Senden-An-Vorlagen](./senden_an_vorlagen.md)
