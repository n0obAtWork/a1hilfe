# Stammdaten Auswertungspositionen

<!-- source: https://amic.de/hilfe/stammdatenauswertungspositione.htm -->

Hauptmenü > Abschlussarbeiten > Umsatzsteuer > Auswertungspositionen

Direktsprung **[FIAWP]**

Bei der Einrichtung der Auswertungspositionen sollte man das Formular der Umsatzsteuervoranmeldung immer vor Augen haben. Wenn der zugelassene Vordruck verwendet werden soll, müssen für alle Kennzahlen (fett gedruckte Zahlen auf dem Formular) Auswertungspositionen eingerichtet werden - unabhängig davon, ob in Ihrem Betrieb diese Steuervorfälle stattfinden oder nicht -, da vor dem Ausdruck ein Testlauf stattfindet, der die Stammdaten auf Vollständigkeit prüft. Dieser Test kann auch jederzeit über den [Fibureorganisator](../../fibu_reorganisator/fibu_reorganisator_allgemein.md) (Direktsprung **[FIREO]**) mit "Test Stammdaten" aufgerufen werden.

![Ein Bild, das Text, Screenshot, Software, Display enthält. Automatisch generierte Beschreibung](../../../ImagesExt/image8_565.png "Ein Bild, das Text, Screenshot, Software, Display enthält. Automatisch generierte Beschreibung")

Die Sortierung gibt an, in welcher Reihenfolge die Daten bei der Auswertung auf dem Bildschirm dargestellt werden. Wenn man sich bei dem Feld Sortierung an die Positionsnummer links auf dem Umsatzsteuervoranmeldungs-Formular hält, ist es leicht, die Reihenfolge so wie sie auf dem Formular vorgegeben ist, einzurichten.

Der Text (Bezeichnung) ist für den Vordruck nicht von Belang, jedoch erleichtert eine korrekte Bezeichnung die Übersicht über die Einrichtung. In dem Menü "Umsatzsteuerwerte" (Direktsprung UVA) werden diese Texte bei der Auswertung nach Auswertungspositionen mit angezeigt.

Die Oberposition dient zur Summierung der einzelnen Zeilen für die Summenfelder in den Zeilen 53,62,65 und 67 (bezogen auf das Umsatzsteuer-voranmeldungs- Formular 2007). Soll der zugelassene Vordruck verwendet werden, braucht hier nichts eingetragen zu werden, da die Summen über die Kennziffern automatisch gebildet werden. Ansonsten müssen dort für alle Zeilen mit einem Feld für die Steuer eine existierende Auswertungsposition eingetragen werden. Die Auswertungspositionen für die Zeilen 53 muss dann auch wieder eine Oberposition eingetragen haben, die die Summe in der Zeile 62 darstellt. Die Zeile 67 (Kennzahl 83) weist somit das Ergebnis der Umsatzsteuervoranmeldung aus. 

Die Kennzahlen Bemessungsgrundlage und Steuer müssen die den Zeilen zugeordneten Kennzahlen enthalten. Wenn Sie Ihre Einrichtung an das Formular angelehnt haben, muss also für die Auswertungsposition 21 die Kennzahl für die Bemessungsgrundlage 41 lauten und die für die Steuer kann leer (0) bleiben. Die einzige Kennzahl, die nicht existieren muss, ist 83, da die Summen direkt im Vordruck gebildet werden.

 Die Einrichtung der Auswertungspositionen könnte wie folgt aussehen. Diese Kennziffern beziehen sich auf das Umsatzsteuer-Voranmeldungsformular des Jahres 2007

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3"></td>
        <td colspan="3">
          <p>Kennziffern</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zeile</p>
        </td>
        <td>
          <p>&nbsp;Bezeichnung</p>
        </td>
        <td>
          <p>Oberp</p>
        </td>
        <td colspan="2">
          <p>Bem.<br>Grund.</p>
        </td>
        <td>
          <p>Steuer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>210</p>
        </td>
        <td>
          <p>EU Lief. an Abnehmer mit Ust-IdNr.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>41</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>220</p>
        </td>
        <td>
          <p>EU Lief. neuer Fahrzeuge ohne Ust-IdNr.</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>44</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>230</p>
        </td>
        <td>
          <p>EU Lief. n. Fahrz. außerh. Unternehmen&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>49</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>240</p>
        </td>
        <td>
          <p>weitere Steuerfreie Umsätze&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>43</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>250</p>
        </td>
        <td>
          <p>Umsätze nach §4 Nr. 8 bis 28 UStG&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>48</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>270</p>
        </td>
        <td>
          <p>zum Steuerumsatz von 19 v. H.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>81</p>
        </td>
        <td colspan="2">
          <p>81</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>290</p>
        </td>
        <td>
          <p>zum Steuersatz von 7 v.H.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>86</p>
        </td>
        <td colspan="2">
          <p>86</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>300</p>
        </td>
        <td>
          <p>Umsätze, mit anderen Steuersätzen&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>35</p>
        </td>
        <td colspan="2">
          <p>36</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>310</p>
        </td>
        <td>
          <p>Lief. an Abnehmer mit Ust-IdNr.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>77</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>320</p>
        </td>
        <td>
          <p>Umsätze mit Steuer nach §24 UStG&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>76</p>
        </td>
        <td colspan="2">
          <p>80</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>340</p>
        </td>
        <td>
          <p>Erwerbe nach §4b UStG&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>91</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>350</p>
        </td>
        <td>
          <p>zum Steuersatz von 19 v.H.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>89</p>
        </td>
        <td colspan="2">
          <p>89</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>360</p>
        </td>
        <td>
          <p>zum Steuersatz von 7 v.H.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>93</p>
        </td>
        <td colspan="2">
          <p>93</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>370</p>
        </td>
        <td>
          <p>zu anderen Steuersätzen&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>95</p>
        </td>
        <td colspan="2">
          <p>98</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>380</p>
        </td>
        <td>
          <p>Lieferungen <b>ohne </b>Ust-IdNr.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>94</p>
        </td>
        <td colspan="2">
          <p>96</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>400</p>
        </td>
        <td>
          <p>Innergemeinschaftliches Dreiecksgeschäft</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>42</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>410</p>
        </td>
        <td>
          <p>Steuerpflichtige Umsätze im Sinne des § 13b Abs 1. Satz 1 Nr. 1 bis 5</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>60</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>415</p>
        </td>
        <td>
          <p>Nicht steuerbare sonstige Leistungen gem. § 18b Satz 1 Nr. 2 UStG</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>21</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>420</p>
        </td>
        <td>
          <p>Im Inland nicht steuerbare Umsätze&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>45</p>
        </td>
        <td colspan="2">
          <p>0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>470</p>
        </td>
        <td>
          <p>Im Inland steuerpfl. sonstige Leistungen von im übrigen Gemeinschaftsgebiet ansässigen Unternehmern</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>46</p>
        </td>
        <td colspan="2">
          <p>47</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>480</p>
        </td>
        <td>
          <p>Leistungen eines im Ausland ansässigen Unternehmers&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>52</p>
        </td>
        <td colspan="2">
          <p>53</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>490</p>
        </td>
        <td>
          <p>Lieferungen sicherungsübereigneter Gegenstände und Umsätze die unter das GrESTG fallen</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>73</p>
        </td>
        <td colspan="2">
          <p>74</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>500</p>
        </td>
        <td>
          <p>Bauleistungen eines im Inland ansässigen Unternehmers</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>84</p>
        </td>
        <td colspan="2">
          <p>85</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>520</p>
        </td>
        <td>
          <p>Wechsel der Besteuerungsform/Nachsteuer</p>
        </td>
        <td>
          <p>530</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>65</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>530</b></p>
        </td>
        <td>
          <p><b>Umsatzsteuer&nbsp;</b></p>
        </td>
        <td>
          <p><b>620</b></p>
        </td>
        <td>
          <p><b>0</b></p>
        </td>
        <td colspan="2">
          <p><b>0</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>550</p>
        </td>
        <td>
          <p>Vorsteuerbeträge aus Rechnungen von anderen Unternehmern...</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>66</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>560</p>
        </td>
        <td>
          <p>Vorsteuerbeträge aus dem innergemeinschaftlichen Erwerb von Gegenständen.</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>61</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>570</p>
        </td>
        <td>
          <p>Entrichtete Einfuhrumsatzsteuer(§15 Abs. 1 Nr. 3 UstG )</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>62</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>580</p>
        </td>
        <td>
          <p>Vorsteuerbeträge aus Leistungen im Sinne des § 13b Abs.1 UstG.</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>67</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>590</p>
        </td>
        <td>
          <p>nach allgemeinen Durchschnittssätzen&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>63</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>600</p>
        </td>
        <td>
          <p>Berichtigung des Vorsteuerabzugs&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>64</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>610</p>
        </td>
        <td>
          <p>Vorsteuerabzug Innergemeinschaftliche L.</p>
        </td>
        <td>
          <p>620</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>59</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>620</b></p>
        </td>
        <td>
          <p><b>Verbleibender Betrag&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</b></p>
        </td>
        <td>
          <p><b>650</b></p>
        </td>
        <td>
          <p><b>0</b></p>
        </td>
        <td colspan="2">
          <p><b>0</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>640</p>
        </td>
        <td>
          <p>geschuldete Steuerbeträge&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>650</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>69</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>650</b></p>
        </td>
        <td>
          <p><b>Umsatzsteuervorauszahlung/Überschuss&nbsp;&nbsp;&nbsp;&nbsp;</b></p>
        </td>
        <td>
          <p><b>670</b></p>
        </td>
        <td>
          <p><b>0</b></p>
        </td>
        <td colspan="2">
          <p><b>0</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>570</p>
        </td>
        <td>
          <p>Anrechnung der Sondervorauszahlung&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
        </td>
        <td>
          <p>670</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td colspan="2">
          <p>39</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>670</b></p>
        </td>
        <td>
          <p><b>Verbleibende Umsatzsteuer-Vorauszahlung</b></p>
        </td>
        <td>
          <p><b>0</b></p>
        </td>
        <td>
          <p><b>0</b></p>
        </td>
        <td colspan="2">
          <p><b>83</b></p>
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
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Die in Fettschrift dargestellten Zeilen dienen nur für die Summendarstellung auf dem Bildschirm. Sie werden weder im Formular noch von Elster verwendet.

Die Sondervorauszahlung – also Kennziffer Steuer 39 – wird nur noch in dem Zeitraum ausgewertet, der den Dezember mit berücksichtig (also z.B. bei vierteljährlicher Voranmeldung nur im letzten Quartal). Dann werden die Werte des gesamten Jahres herangezogen, so dass die Buchung in dem Zeitraum erfolgen kann, in dem die Sondervorauszahlung erfolgt. Dies ist für das neue Verfahren ELSTER notwendig, da dort auch die Anmeldung zur Sondervorauszahlung erfolgen kann.
