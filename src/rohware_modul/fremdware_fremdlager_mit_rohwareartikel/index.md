# Fremdware-/Fremdlager mit Rohwareartikel

<!-- source: https://amic.de/hilfe/fremdwarefremdlagermitrohwarea.htm -->

In A.eins können Fremdware- bzw. Fremdlager-Positionen, die mittels der [Vorfakturierung](./vorfakturierung.md) gebildet werden, nun auch im Rohwarenbearbeitungsmodul abgewickelt werden. Dazu wird bei der Vorfakturierung eines Rohwareartikels (Artikel mit eingetragener Rohwarengruppe) für den anzulegenden Fremdkontrakt eine Reihe zusätzlicher Informationen hinterlegt ( Brutto-/Nettomengen-Kennzeichen, Final-/Weltmarkt-/Mindestpreis) [Zusatzinfo in Fremdkontrakt].

Ein derartiger Kontrakt kann bei der Erfassung von Rohware-Belegen für die Lieferposition ausgewählt werden. Rohwarebelege mit Bezug zu einem Fremdkontrakt können aber auch durch Rohware-Wandlung von entsprechenden Normal-Lieferscheinen sowie durch die Belegerzeugung aus der Rohware-Waagen-Schnittstelle erstellt werden.

Rohware-Belege mit Fremdkontrakt werden wie andere Rohware-Belege bearbeitet: Sie können korrigiert, abgerechnet, gedruckt, finalisiert, storniert, gebucht etc. werden.

AUSNAHME: Es können keine Abschlag- und Folgeabschlagbelege erstellt werden. Die vorfakturierte Rechnung, die den Fremdkontrakt erzeugt hat, stellt aus der Sicht der Rohwarenbearbeitung einen anzurechnenden Abschlag dar.

Dieser Abschlagbetrag wird bei der Abrechnung der zugehörigen Finalbelege auf den Betrag der jeweiligen Lieferposition angerechnet, bis er ‚aufgebraucht’ ist.

Bei mehreren Abrechnungen zu einem Fremdkontrakt ist daher i.d.R. der Buchwert der Anlieferpositionen der ersten Abrechnungen 0,00, der auf den Beleg anzurechnende Abschlag jeweils der Abrechnungswert der Anlieferzeile und der Beleg-Gesamtbetrag bei bestehenden Kostenpositionen negativ.  
Erst wenn der Abschlag (vorfakturierter Betrag) ‚aufgebraucht’ ist, ändert sich dieses.

Es macht daher ggf. Sinn, eine Reihe von Finalabrechnungen zu einem Fremdkontrakt per Sammelabrechnung zu behandeln.

Zur besseren Orientierung wurde in den für die Rohwarenbearbeitung relevanten Auswahllisten eine zusätzliche Spalte aufgenommen, die Auskunft darüber gibt, ob der jeweilige Beleg zu einem Fremdkontrakt gehört oder nicht. Dieses Kennzeichen ist auch in den zugehörigen Bereichsauswahlen einstellbar.

Mittels einer [speziellen Auswahlvariante](./auswahlvariante_vorfakt_kontrakte/index.md) kann der aktuelle Stand der Fremdkontrakte gelistet werden.

<p class="siehe-auch">Siehe auch:</p>

- [Auswahlvariante ‚Vorfakt. Kontrakte’](./auswahlvariante_vorfakt_kontrakte/index.md)
- [Vorfakturierung](./vorfakturierung.md)
- [Zusatzinfo in Fremdkontrakt](./zusatzinfo_in_fremdkontrakt.md)
- [Abbuchungsmenge (brutto/netto)](./abbuchungsmenge_brutto_netto.md)
- [Finalabrechnungspreis](./finalabrechnungspreis.md)
- [Abschlagpreis](./abschlagpreis.md)
- [Mindestpreis](./mindestpreis.md)
- [Weltmarktpreis](./weltmarktpreis.md)
