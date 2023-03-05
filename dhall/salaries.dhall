let Salary = ./types/Salary.dhall

let salaries =
      [ { company = "Tailscale"
        , title = "Member of Technical Staff"
        , start_date = "2022-09-06"
        , end_date = ""
        , salary = "EUR 188 240 + options"
        , how_i_left = "N/A"
        , note = "options, 1 year cliff, 4 year vesting"
        }
      , { company = "G-Research"
        , title = "Site Reliability Engineer - Engineer II"
        , start_date = "2022-01-01"
        , end_date = "2022-08-11"
        , salary = "GBP 110 000 + GBP 60 000 (Bonus)"
        , how_i_left = "quit"
        , note = "bonus is for 2021"
        }
      , { company = "G-Research"
        , title = "Site Reliability Engineer - Engineer I"
        , start_date = "2021-01-01"
        , end_date = "2021-12-31"
        , salary = "GBP 90 000 + GBP 33 000 (Bonus)"
        , how_i_left = "raise"
        , note = "bonus is for 2020"
        }
      , { company = "G-Research"
        , title = "Site Reliability Engineer - Engineer I"
        , start_date = "2019-09-02"
        , end_date = "2020-12-31"
        , salary = "GBP 85 000 + GBP 7500 (Bonus)"
        , how_i_left = "raise"
        , note = "bonus is for 2019"
        }
      , { company = "ESA"
        , title = "Young Graduate Trainee"
        , start_date = "2017-09-01"
        , end_date = "2019-08-31"
        , salary = "EUR 2720/month"
        , how_i_left = "one + one year contract"
        , note = "tax exempt"
        }
      , { company = "Knowit Objectnet"
        , title = "Summer job"
        , start_date = "2016-06-01"
        , end_date = "2016-08-15"
        , salary = "NOK 225/hour"
        , how_i_left = "end of summer"
        , note = "job offer, NOK 460 000"
        }
      , { company = "Uninett Sigma 2"
        , title = "Summer job"
        , start_date = "2015-06-26"
        , end_date = "2015-08-31"
        , salary = "NOK 185/hour"
        , how_i_left = "end of summer"
        , note = ""
        }
      ]

in  salaries
