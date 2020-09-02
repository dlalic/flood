import { Landscape } from 'wasm'
import Chart from 'chart.js'

window.onload = function () {
  var form = document.querySelector('form')
  form.onsubmit = submitted.bind(form)
}

function submitted (event) {
  event.preventDefault()
  update()
}

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
      backgroundColor: '#D2691E',
      barPercentage: 1.0,
      categoryPercentage: 1.0,
    },
    {
      label: 'Water level',
      backgroundColor: '#F0F8FF',
      barPercentage: 1.0,
      categoryPercentage: 1.0,
    }
  ]
}

const update = () => {
  const input = document.getElementById('input').value
  const landscape = Landscape.new(input)
  landscape.update()
  const count = landscape.elevations_count()
  barChartData.labels = [];
  barChartData.datasets[0].data = [];
  barChartData.datasets[1].data = [];
  for (let i = 0; i < count; i++) {
    barChartData.labels[i] = i
    barChartData.datasets[0].data[i] = landscape.elevation(i)
    barChartData.datasets[1].data[i] = landscape.water_level(i)
  }
  chart.data = barChartData
  chart.update()
}

update()
