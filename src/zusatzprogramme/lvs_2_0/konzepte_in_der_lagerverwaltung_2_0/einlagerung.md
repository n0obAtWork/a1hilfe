# Einlagerung

<!-- source: https://amic.de/hilfe/_lvs20_einlagerung.htm -->

Es gibt verschiedene Einlagerungsmöglichkeiten:

1. Manuell – Hier bestimmt der Lagermitarbeiter selbst, wohin die Ware eingelagert wird

2. Nach Vorschlag – Hier wird bei der Erstellung eines Ladeträgers im Wareneingang durch Vorgangsimport im LVS-Kontrollmakro ein Fahrauftrag zu einer Lokalität erzeugt.

```sql
public void
After_Import(ImportVorgStamm ivs)
{
  ImportVorgPosition ivp =
ivs._ImportVorgPosition[0];

  int lokTyp =
D.GetExecuteScalar(0, "select lokalitaetstyp from
lvs_lokalitaeten lk where lokalitaetsnr = ? ",
ivp._ImportVorgPositionLVS[0].LokalitaetsNr);

  if (lokTyp ==
44) // Produktion Fertigware
  {
    D.ExecuteNonQuery("call p_DEMO_Einlagerstrategie(?,?,7000);",
ivp.UebernahmeId, ivp.SatzId);
  }
  if (lokTyp ==
10) // Wareneingang
  {
    D.ExecuteNonQuery("call p_DEMO_Einlagerstrategie(?,?,7000);",
ivp.UebernahmeId, ivp.SatzId);
  }
}
```

Eine private Einlagerungsstrategie (Hier p_DEMO_Einlagerstrategie) optimiert hier Wege oder Befüllungsgrad des Lagers.

**Empfohlener Arbeitsablauf Scanner:**

• Scan der NVE

o Anzeige der NVE-Info ggf. mit Fahrauftrags-Vorschlag

• Scan der neuen Lokalität

o Erzeugen einer Ladeträgerbewegung im VIMP
