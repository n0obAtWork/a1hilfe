# Formulareinrichtung für den Kontraktdruck

<!-- source: https://amic.de/hilfe/_formulareinrichtungf.htm -->

Folgende Formulartypen sind derzeit innerhalb der Kontraktverwaltung vorgesehen:

- Kontrakt (auch Kontraktbestätigung genannt)
- Kontrakt-Erledigungsschreiben
- Kontrakt-Stornobeleg
- Kontrakt-Erinnerung (oder -mahnung)
- Andienung
- Freistellung
- Andienung und Freistellung zugleich

Momentan sind lediglich der Kontraktdruck, -erledigung und -storno implementiert.

Des Weiteren sollte beachtet werden, dass der Kontraktdruck auch an die „[Vorgangsdruckklassen](../firmenstamm/druckereinrichtung/vorgangsdruckklassen.md)“ angeschlossen werden kann.

<p class="just-emphasize">Sinn der „Kontrakt-Varianten"</p>

Da die optische Gestaltung eines Kontraktes nicht, wie bei einem Vorgang, durch die Reihenfolge der Eingabe, sondern nach logischen Gesichtspunkten gegliedert werden soll, muss dem System ebendiese Reihenfolge mitgeteilt werden. Dies geschieht in der Kontrakt-Variante, der der Kontrakt jeweils zu Druckzwecken zuzuordnen ist. Die Druckvariante steuert also nicht nur, welche Formulareinrichtung verwendet werden soll, sondern auch die Reihenfolge und den Umfang der „Bereiche", in die ein Kontrakt gegliedert werden soll.

Innerhalb einer Kontrakt-Variante können folgende Bereiche aktiviert werden:

- Artikelposition (nur Artikeldaten mit Folgetextzeilen)
- Artikel mit Mengenzeilen (je Artikel über alle Zeiträume)
- Artikel mit Preiszeilen (je Artikel über alle Zeiträume)
- Artikel mit Mengen- und Preiszeilen (jeweils je Artikel über alle Zeiträume)
- Abnahme- oder Mengenzeitraum (nur Gesamtmenge)
- Abnahmezeitraum mit Artikelzeilen (je Zeitraum über alle Artikel)
- Bepreisungszeitraum (nur Zeitraumgrenzen)
- Bepreisungszeitraum mit Artikelzeilen (je Zeitraum über alle Artikel)
- Abnahmekunden (nur bei Gruppenkontrakten sinnvoll)
- Paritätsdaten
- Zu-/Abschläge (noch nicht implementiert)
- Kontraktpartien (noch nicht implementiert)
- Bewegungszeile (z. B. für Erledigungsschreiben)
- Festtext (Textbaustein)
- Zahlungsbedingungen
- Leerzeile

Folgende Formularbereiche (Direktsprung **[FRMB]**) können in der Formulareinrichtung berücksichtigt werden:

- Kontrakt-Kopf
- Kontrakt-Kopf für Folgeseiten
- Kontrakt-Fuß
- Kontrakt-Abschluss
- Artikelposition
- Artikel-Folgetextzeile
- Artikel-Zeitraummenge
- Artikel-Zeitraumpreis
- Abnahme- oder Mengenzeitraum
- Abnahmezeitraum-Artikelmenge
- Bepreisungszeitraum
- Bepreisungszeitraum-Artikelpreis
- Kontrakt-Abnahmekunden (nur bei Gruppenkontrakten sinnvoll)
- Kontrakt-Paritätsdaten
- Kontrakt-Zu-/Abschläge (noch nicht implementiert)
- Kontrakt-Textzeile (Textbaustein)
- Kontrakt-Bereichsüberschrift
- Kontrakt-Zahlungsbedingung
- Kontrakt-Leerzeile
- Druckabbruch-Hinweis

Achtung: Damit ein Kontrakt-Kopf für Folgeseiten gedruckt werden kann, muss ein Kontrakt-Fuß eingerichtet werden.

<p class="just-emphasize">Einrichtung des jeweiligen Bereichs</p>

Da der Kontraktdruck eine sehr viel allgemeinere Formularverwaltung verwendet als das Vorgangswesen, können hier nicht die dort üblichen Positionen verwendet werden. Es gibt nur die Einstellungen „Festtext“, „Variable als Text“, „Variable als Zahl“, „Variable als Datum“ und (wo sinnvoll) „Variable als Anschrift“ sowie die Positionen „Ktr_Versandadresse oder Hauptadresse“ (3210) und „Ktr_Versandadresse oder Hauptadresse“ (3211). Um WELCHE Variable des jeweiligen Typs es sich genau handeln soll, muss in der Spalte „Text“ hinterlegt werden.

<p class="just-emphasize">Im Kontraktkopf (auch Folgeseite) derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Kontraktnummer | KtrNummer | 4 |
| Kontraktdatum | KtrDatum | 5 |
| Kontraktbezeichnung | KtrBezeich | 3 |
| Anfang der Laufzeit | KtrAbDatum | 5 |
| Ende der Laufzeit | KtrBisDatum | 5 |
| Mengeneinheits-Kurztext | ME_Kurztext | 3 |
| Mengeneinheits-Bezeichnung | ME_Bezeich | 3 |
| Währungs-Kurztext | WaehrKurztext | 3 |
| Kontraktgruppennummer | KtrGrupNummer | 4 |
| Kontraktgruppenbezeichnung | KtrGrupBezeich | 3 |
| Kundennummer des Hauptkunden | KundNummer | 4 |
| Bezeichnung des Hauptkunden | KundBezeich | 3 |
| Vetretergruppennummer (ggf. des Hauptkunden) | VertGrNummer | 4 |
| Vertretergruppenbezeichnung | VertGrBezeich | 3 |
| Zahlungsbedingungsnummer (ggf. des Hauptkunden) | ZahlBedNummer | 4 |
| Zahlungsbedingungsbezeichnung | ZahlBedBezeich | 3 |
| Anschrift des Hauptkunden (bzw. manuelle Kontraktanschr.) | AdressId | 6 |
| Ktr_Versandadresse oder Hauptadresse<br> | | 3210 |
| Ktr_Versandadresse oder Hauptadresse<br> | | 3211 |
| Anschrift der Filiale <br> | AdressIdFiliale | 6 |
| Identifikation des erfassenden Bedieners | BedienerIdNeu | 4 |
| Kurzbezeichnung des erfassenden Bedieners | BedienerKurzNeu | 3 |
| Langbezeichnung des erfassenden Bedieners | BedienerNameNeu | 3 |
| Reportsatz | KtrReport | 4 |
| Report-Bezeichnung | KtrReportBezeich | 3 |
| Report-Anfangsdatum | KtrReportAbDatum | 5 |
| Report-Periode (Report je x Tage) | KtrReportJeTage | 4 |
| Abgeschlossene Gesamtmenge | GesamtMenge | 4 |
| Erledigte Gesamtmenge | IstMenge | 4 |
| Offene Gesamtmenge | RestMenge | 4 |
| Abgeschlossener Gesamtwert | GesamtWert | 4 |
| Erledigter Gesamtwert | IstWert | 4 |
| Offener Gesamtwert | RestWert | 4 |

<p class="just-emphasize">In der ersten Zeile eines JEDEN Hauptteil-Bereichs verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Beschriftung der ersten Bereichszeile | KtrVariTxVorText | 3 |
| Interne Identifikation des Kontrakts (für SQLK) | KtrId | 4 |
| Tagesdatum des Drucks | Tagesdatum | 5 |

<p class="just-emphasize">In der Artikelposition derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Artikelposition im Kontrakt (für SQLK) | KtrArtiPosit | 4 |
| Artikelnummer | ArtikelNummer | 3 |
| Artikelbezeichnung | ArtikelBezeich | 3 |
| Artikeltext (erste Zeile) | ArtTextText | 3 |
| Anfang der Kontraktlaufzeit | KtrAbDatum | 5 |
| Ende der Kontraktlaufzeit | KtrBisDatum | 5 |
| Abgeschlossene Gesamtmenge des Artikels | KtrArtiGesaMenge | 4 |
| Abgeschlossener Gesamtwert des Artikels | KtrArtiGesaWert | 4 |
| Aktuelle erfüllte Menge des Artikels | KtrArtiGesaIstMe | 4 |
| Aktueller erfüllter Wert des Artikels | KtrArtiGesaIstWe | 4 |
| Aktuelle Restmenge des Artikels | KtrArtiGesaResMe | 4 |
| Aktueller Restwert des Artikels | KtrArtiGesaResWe | 4 |
| Mengeneinheits-Kurztext | ME_Kurztext | 3 |
| Mengeneinheits-Bezeichnung | ME_Bezeich | 3 |
| (Erster) Kontraktpreis | KtrPreis | 4 |
| Zugehörige Preiseinheit | KtrPreisEinh | 4 |
| Kurztext der Preis-Mengeneinheit | ME_KurztextPreis | 3 |
| Währungs-Kurztext | WaehrKurztext | 3 |
| Allgemeiner Wert | KtrArtiAnwWert | 4 |
| Bemerkung | KtrArtiAnwBemerk | 3 |
| Frachtzu-/abschlag | Fracht | 4 |
| Frachtsatz | FrachtSatz | 4 |
| Tradebasis | Matif | 4 |
| Handling | Handling | 4 |
| Pricing | Pricing | 3 |
| Future | Future | 4 |
| Hedge Monat | HedgeMonat | 5 |
| Artikelidentifikation des Hedgeartikels | HedgeArtikelId | 3 |
| Nachhaltigkeitsstatus | nachhaltigkeitsartikel | 3 |
| THG – Anbauwert | thgwertanbau | 4 |
| THG – Lieferungswert | thgwertlieferung | 4 |
| THG – Verarbeitungswert | thgwertverarbeitung | 4 |
| Planlieferdatum | KtrArtiPlanLiefDatum | 3 |
| Planlieferzeit | KtrArtiPlanLiefZeit | 3 |

Zusätzlich lassen sich hier auch die [Rohwarepreise](./formulareinrichtung_fuer_den_kontraktdruck.md#formular_rohwarepreise) einrichten.

<p class="just-emphasize">In der Artikel-Zeitraummenge derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Artikelposition im Kontrakt (für SQLK) | KtrArtiPosit | 4 |
| Anfang Abnahmezeitraum | KtrMeZrAbDatum | 5 |
| Ende Abnahmezeitraum | KtrMeZrBisDatum | 5 |
| Textuelle Bezeichnung des Zeitraums (Monatsname) | ZeitraumText | 3 |
| Abgeschlossene Menge des Artikels im Zeitraum | SollMenge | 4 |
| Abgeschlossener Wert des Artikels im Zeitraum | SollWert | 4 |
| Erfüllte Menge des Artikels im Zeitraum | IstMenge | 4 |
| Erfüllter Wert des Artikels im Zeitraum | IstWert | 4 |
| Restmenge des Artikels im Zeitraum | RestMenge | 4 |
| Restwert des Artikels im Zeitraum | RestWert | 4 |
| Mengeneinheits-Kurztext | ME_Kurztext | 3 |
| Mengeneinheits-Bezeichnung | ME_Bezeich | 3 |
| Parallel gültiger Kontraktpreis | KtrPreis | 4 |
| Zugehörige Preiseinheit | KtrPreisEinh | 4 |
| Kurztext der Preis-Mengeneinheit | ME_KurztextPreis | 3 |
| Währungs-Kurztext | WaehrKurztext | 3 |

Zusätzlich lassen sich hier auch die [Rohwarepreise](./formulareinrichtung_fuer_den_kontraktdruck.md#formular_rohwarepreise) einrichten.

<p class="just-emphasize">Im Artikel-Zeitraumpreis derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Artikelposition im Kontrakt (für SQLK) | KtrArtiPosit | 4 |
| Anfang Bepreisungszeitraum | KtrPrZrAbDatum | 5 |
| Ende Bepreisungszeitraum | KtrPrZrBisDatum | 5 |
| Textuelle Bezeichnung des Zeitraums (Monatsname) | ZeitraumText | 3 |
| Kontraktpreis | KtrPreis | 4 |
| Zugehörige Preiseinheit | KtrPreisEinh | 4 |
| Kurztext der Preis-Mengeneinheit | ME_KurztextPreis | 3 |
| Bezeichnung der Preis-Mengeneinheit | ME_BezeichPreis | 3 |
| Währungs-Kurztext | WaehrKurztext | 3 |

Zusätzlich lassen sich hier auch die [Rohwarepreise](./formulareinrichtung_fuer_den_kontraktdruck.md#formular_rohwarepreise) einrichten.

<p class="just-emphasize">In der Mengenzeitraum-Position derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Anfang Abnahmezeitraum | KtrMeZrAbDatum | 5 |
| Ende Abnahmezeitraum | KtrMeZrBisDatum | 5 |
| Textuelle Bezeichnung des Zeitraums (Monatsname) | ZeitraumText | 3 |
| Abgeschlossene Gesamtmenge im Zeitraum | SollMenge | 4 |
| Abgeschlossener Gesamtwert im Zeitraum | SollWert | 4 |
| Erfüllte Menge im Zeitraum | IstMenge | 4 |
| Erfüllter Wert im Zeitraum | IstWert | 4 |
| Restmenge im Zeitraum | RestMenge | 4 |
| Restwert im Zeitraum | RestWert | 4 |
| Mengeneinheits-Kurztext | ME_Kurztext | 3 |
| Mengeneinheits-Bezeichnung | ME_Bezeich | 3 |

<p class="just-emphasize">In der Zeitraum-Artikelmenge derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Artikelposition im Kontrakt (für SQLK) | KtrArtiPosit | 4 |
| Artikelnummer | ArtikelNummer | 3 |
| Artikelbezeichnung | ArtikelBezeich | 3 |
| Anfang Abnahmezeitraum | KtrMeZrAbDatum | 5 |
| Ende Abnahmezeitraum | KtrMeZrBisDatum | 5 |
| Textuelle Bezeichnung des Zeitraums (Monatsname) | ZeitraumText | 3 |
| Abgeschlossene Menge des Artikels im Zeitraum | SollMenge | 4 |
| Abgeschlossener Wert des Artikels im Zeitraum | SollWert | 4 |
| Erfüllte Menge des Artikels im Zeitraum | IstMenge | 4 |
| Erfüllter Wert des Artikels im Zeitraum | IstWert | 4 |
| Restmenge des Artikels im Zeitraum | RestMenge | 4 |
| Restwert des Artikels im Zeitraum | RestWert | 4 |
| Mengeneinheits-Kurztext | ME_Kurztext | 3 |
| Parallel gültiger Kontraktpreis | KtrPreis | 4 |
| Zugehörige Preiseinheit | KtrPreisEinh | 4 |
| Kurztext der Preis-Mengeneinheit | ME_KurztextPreis | 3 |
| Währungs-Kurztext | WaehrKurztext | 3 |

Zusätzlich lassen sich hier auch die [Rohwarepreise](./formulareinrichtung_fuer_den_kontraktdruck.md#formular_rohwarepreise) einrichten.

<p class="just-emphasize">In der Preiszeitraum-Position derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Anfang Bepreisungszeitraum | KtrPrZrAbDatum | 5 |
| Ende Bepreisungszeitraum | KtrPrZrBisDatum | 5 |
| Textuelle Bezeichnung des Zeitraums (Monatsname) | ZeitraumText | 3 |

<p class="just-emphasize">Im Bepreisungszeitraum-Artikelpreis derzeit verwendbare Daten</p>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Kürzel</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Typ</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelposition im Kontrakt (für SQLK)</p>
        </td>
        <td colspan="2">
          <p>KtrArtiPosit</p>
        </td>
        <td>
          <p>4</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelnummer</p>
        </td>
        <td colspan="2">
          <p>ArtikelNummer</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelbezeichnung</p>
        </td>
        <td colspan="2">
          <p>ArtikelBezeich<u></u></p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Anfang Bepreisungszeitraum</p>
        </td>
        <td colspan="2">
          <p>KtrPrZrAbDatum<u></u></p>
        </td>
        <td>
          <p>5</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ende Bepreisungszeitraum</p>
        </td>
        <td colspan="2">
          <p>KtrPrZrBisDatum</p>
        </td>
        <td>
          <p>5</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Textuelle Bezeichnung des Zeitraums (Monatsname)</p>
        </td>
        <td colspan="2">
          <p>ZeitraumText</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontraktpreis<u></u></p>
        </td>
        <td colspan="2">
          <p>KtrPreis</p>
        </td>
        <td>
          <p>4</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zugehörige Preiseinheit<u></u></p>
        </td>
        <td colspan="2">
          <p>KtrPreisEinh</p>
        </td>
        <td>
          <p>4</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kurztext der Preis-Mengeneinheit</p>
        </td>
        <td colspan="2">
          <p>ME_KurztextPreis</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung der Preis-Mengeneinheit</p>
        </td>
        <td colspan="2">
          <p>ME_BezeichPreis</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Währungs-Kurztext</p>
        </td>
        <td colspan="2">
          <p>WaehrKurztext</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Zusätzlich lassen sich hier auch die [Rohwarepreise](./formulareinrichtung_fuer_den_kontraktdruck.md#formular_rohwarepreise) einrichten.

<p class="just-emphasize">In einer Abnahmekunden-Position derzeit verwendbare Daten</p>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Kürzel</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Typ</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundennummer</p>
        </td>
        <td colspan="2">
          <p>KundNummer</p>
        </td>
        <td>
          <p>4</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundenbezeichnung</p>
        </td>
        <td colspan="2">
          <p>KundBezeich</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundenanschrift</p>
        </td>
        <td colspan="2">
          <p>AdressId<u></u></p>
        </td>
        <td>
          <p>6<u></u></p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">In einer Paritätsposition derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Paritätsnummer | ParitaetNummer | 4 |
| Paritätsbezeichnung | ParitaetBezeich | 3 |
| Paritätsanschrift | AdressId | 6 |

<p class="just-emphasize">In einer Kontraktbewegung derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Kundennummer | KundNummer | 3 |
| Kundenbezeichnung | KundBezeich | 3 |
| Artikelnummer | ArtikelNummer | 3 |
| Artikelbezeichnung | ArtikelBezeich | 3 |
| Bewegungsdatum | KtrBewDatum | 5 |
| Bewegungsmenge | KtrBewMenge | 4 |
| Mengeneinheits-Kurztext | ME_Kurztext | 3 |
| Mengeneinheits-Bezeichnung | ME_Bezeich | 3 |
| Bewegungswert | KtrBewWert | 4 |
| Währungskurs | KtrBewWaehrMulti | 4 |
| Währungs-Kurztext | WaehrKurztext | 3 |
| Einzelpreis | KtrBewPreis | 4 |
| Preiseinheit | KtrBewPreisEinh | 4 |
| Preis-Mengeneinheits-Kurztext | ME_KurztextPreis | 3 |
| Preis-Mengeneinheits-Bezeichnung | ME_BezeichPreis | 3 |
| Parallel gültiger Kontraktpreis | KtrPreis | 4 |

<p class="just-emphasize">In einer Textbausteinzeile derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Textzeile | BemerkText | 3 |

<p class="just-emphasize">In einer Bereichsüberschrift derzeit verwendbare Daten</p>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Kürzel</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Typ</strong></p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Überschrift</p>
        </td>
        <td colspan="2">
          <p>KtrVariTxUeber</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">In einer Zahlungsbedingung derzeit verwendbare Daten</p>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Kürzel</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Typ</strong></p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Zahlungsbedingungsnummer (ggf. des Hauptkunden)</p>
        </td>
        <td colspan="2">
          <p>ZahlBedNummer</p>
        </td>
        <td>
          <p>4</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Zahlungsbedingungsbezeichnung</p>
        </td>
        <td colspan="2">
          <p>ZahlBedBezeich</p>
        </td>
        <td>
          <p>3</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">In einer Leerzeile derzeit verwendbare Daten</p>

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Leerzeile (bewirkt nichts, muss aber da sein!) | Leerzeile | 3 |

<p class="just-emphasize">Rohwarenpreise</p>

Preisfelder die nur für Rohwarekontrakte gelten, diese können in unterschiedlichen Bereichen verwendet werden.

| Feld | Kürzel | Typ |
| --- | --- | --- |
| Abschlagspreis | KtrPrRohAbPreis | 4 |
| Folgeabschlagspreis | KtrPrRohFAbPreis | 4 |
| Mindestpreis | KtrPrRohMinPreis | 4 |
| Weltmarktpreis | KtrPrRohWM_Preis | 4 |
| Abschlagssatz | KtrRohAbschSatz | 4 |

<p class="just-emphasize">Varianten für den Anschriftsdruck</p>

Jede Anschrift kann in derzeit 4 verschiedenen Varianten ausgedruckt werden, deren Form von Anschriftsmaske zu Anschriftsmaske verschieden gestaltbar ist:

Typ 1: Lieferanschrift (kein Postfach, sondern Straße und deren Postleitzahl)

Typ 2: Postanschrift (ist ein Postfach vorhanden, so wird es berücksichtigt)

Typ 3: Kurzanschrift (für Anzeige im Listen etc., wo einzeilig nicht reicht)

Typ 4: Einzeilige Anschrift, speziell für Listen etc.

Diese Anschriftsvariante muss bei der Formulareinrichtung bei Formularposition 6 („Variable als Anschrift“) im Feld „Nachkommastellen“ eingetragen werden!!! Bei Kontrakten empfiehlt sich natürlich die 2 für „Postanschrift"...

7\. Kontraktdruck

Siehe Musterausdrucke

8\. Kontraktabschreibung mit Währungsumrechnung

1\. Währungsstamm (WAE) und Währungskurs (WAK) einrichten

Bitte Kursfaktor (100) beachten wenn z.B. 100,-- DM = 30 Kronen

2\. Steuerungsparameter einstellen auf JA

a) Global = Nr. 10

b) Kontraktwesen = Nr. 22.

3\. Im Deb./Kreditorenstamm unter Gruppen/Klassen Währungstyp beachten!

4\. Im Kontraktstamm unter Stammdaten 2 (Kontrakt) Kontrakt-Währung einstellen
