import { Landscape } from 'wasm'
import Chart from 'chart.js'

const ctx = document.getElementById('canvas')
const chart = new Chart(ctx, {
  type: 'bar',
  options: {
    responsive: true,
    scales: {
      xAxes: [{
        stacked: true
      }],
      yAxes: [{
        stacked: true,
        ticks: {
          stepSize: 0.5
        }
      }]
    },
    animation: {
      duration: 0
    }
  }
})

var barChartData = {
  datasets: [
    {
      label: 'Elevation',
      backgroundColor: '#4f2509',
      barPercentage: 1.0,
      categoryPercentage: 1.0
    },
    {
      label: 'Water level',
      backgroundColor: '#4169E1',
      barPercentage: 1.0,
      categoryPercentage: 1.0
    }
  ]
}

const form = document.getElementById('form')
form.addEventListener('submit', (event) => {
  event.preventDefault()
  update()
})

window.addEventListener('click', event => {
  fill()
})

const landscape = Landscape.new()
const update = () => {
  console.log('Updating')
  const input = document.getElementById('input').value
  landscape.set_elevations(input)
  const count = landscape.elevations_count()
  barChartData.labels = []
  barChartData.datasets[0].data = []
  barChartData.datasets[1].data = []
  for (let i = 0; i < count; i++) {
    barChartData.labels[i] = i
    barChartData.datasets[0].data[i] = landscape.elevation(i)
  }
  chart.data = barChartData
  chart.update()
}

const fill = () => {
  landscape.update()
  const count = landscape.elevations_count()
  barChartData.datasets[1].data = []
  for (let i = 0; i < count; i++) {
    barChartData.datasets[1].data[i] = landscape.water_level(i)
  }
  chart.data = barChartData
  chart.update()
}

update()
