package main

import (
	"encoding/json"
	"fmt"
	"github.com/gorilla/mux"
	"html/template"
	"log"
	"net/http"
	"os"
	"strings"
	"time"
)

const PoligonPath = "https://api.polygon.io"
const TickerPath = PoligonPath + "/v3/reference/tickers"

type Stock struct {
	Name   string `json:"name"`
	Ticker string `json:"ticker"`
}

type SearchResults struct {
	Results []Stock `json:"results"`
}

var client *http.Client
var apiKey string

func ApiKeyQueryString() string {
	return "&apiKey=" + apiKey
}

func GetJson(url string, target interface{}) error {
	resp, err := client.Get(url)
	if err != nil {
		log.Printf("I did have an error! %s\n", err)
		return err
	}

	defer resp.Body.Close()

	return json.NewDecoder(resp.Body).Decode(target)
}

func main() {
	fmt.Println("Go, go, go!")
	client = &http.Client{Timeout: 10 * time.Second}
	apiKey = os.Getenv("APIKEY")

	r := mux.NewRouter()

	r.HandleFunc("/", indexHandler)
	r.HandleFunc("/search", searchHandler)
	r.HandleFunc("/values/{ticker}", tickerHandler)

	r.HandleFunc("/styles/bootstrap.min.css", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "static/bootstrap.min.css")
	})
	r.HandleFunc("/static/htmx.min.js", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "static/htmx.min.js")
	})
	http.Handle("/", r)

	log.Fatal(http.ListenAndServe(":3000", nil))

}

func indexHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("views/index.html"))
	tmpl.Execute(w, nil)
}

func searchHandler(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("ticker")
	searchResults := SearchTicker(searchTerm)
	tmpl := template.Must(template.ParseFiles("views/results.html"))
	tmpl.Execute(w, searchResults)
}

func tickerHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("views/values.html"))

	ticker := mux.Vars(r)["ticker"]
	tickerDailyValues := GetDailyValues(ticker)

	tmpl.Execute(w, tickerDailyValues)
}

func SearchTicker(ticker string) SearchResults {
	url := TickerPath + "?" + "ticker=" + strings.ToUpper(ticker) + ApiKeyQueryString()
	data := SearchResults{}

	err := GetJson(url, &data)
	if err != nil {
		log.Printf("I did have an error! %s\n", err)
	} else {
		log.Printf("I have so many results: %d\n", len(data.Results))
		if len(data.Results) > 0 {
			log.Printf("This is the first result: %s\n", data.Results[0].Name)
		}
	}
	return data
}

const DailyValuesPath = PoligonPath + "/v1/open-close"

type Values struct {
	Open float64 `json:"open"`
	High float64 `json:"high"`
	Low  float64 `json:"low"`
}

type TickerDailyValues struct {
	Ticker string
	Values Values
}

func GetDailyValues(ticker string) TickerDailyValues {
	//today := time.Now().Format("01-02-2006")
	//for sake of reproducibility, I use a fixed date.
	//Later on, I might extend the solution to query for data of a particular day.
	today := "2023-01-09"
	url := DailyValuesPath + "/" + strings.ToUpper(ticker) + "/" + today + "?adjusted=true" + ApiKeyQueryString()
	data := Values{}
	err := GetJson(url, &data)
	if err != nil {
		log.Printf("I did have an error! %s\n", err)
		return TickerDailyValues{}
	} else {
		return TickerDailyValues{
			Ticker: ticker,
			Values: data,
		}
	}
}
