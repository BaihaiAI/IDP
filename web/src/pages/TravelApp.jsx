import React from 'react'
import TravelHeader from "@/layout/travelLayout/travelHeader"
import TravelContent from "@/layout/travelLayout/travelContent"
import TravelFooter from "@/layout/travelLayout/travelFooter"


function TravelApp(props) {
  return (
    <div>
      <TravelHeader />
      <TravelContent />
      <TravelFooter />
    </div>
  )
}

export default TravelApp
