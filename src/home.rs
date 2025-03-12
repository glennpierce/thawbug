use leptos::prelude::*;

use thaw::{Checkbox, ConfigProvider, Input, Scrollbar};

//use crate::components::aggregate::{Aggregate, AggregationCombobox};
//use crate::components::granularity::{Granularity, GranularityCombobox};
//use crate::components::aggregate::AggregationCombobox;
use crate::components::report_style::ReportStyleCombobox;
use crate::report::{PointRef, Report};

#[derive(Clone, Debug)]
pub struct ReportForm {
    pub name: RwSignal<String>,
    pub enabled: RwSignal<bool>,
    pub report_style: RwSignal<Option<String>>,
    pub site_group: RwSignal<Option<String>>, // Used for permissions 
    pub point_refs: RwSignal<Option<Vec<PointRef>>>,
    pub days_back: RwSignal<String>,
   // pub granularity: RwSignal<Granularity>,
   // pub aggregation: RwSignal<Aggregate>,
    pub recipients: RwSignal<String>,
    pub schedule: RwSignal<String>,
    pub tags: RwSignal<String>,
}

impl Default for ReportForm {
    fn default() -> Self {

        ReportForm { 
            name: RwSignal::new(String::new()), 
            enabled: RwSignal::new(false),
            report_style: RwSignal::new(Some(String::from("report_style1"))),
            site_group: RwSignal::new(Some(String::new())), 
            point_refs: RwSignal::new(None),
            days_back: RwSignal::new("7".to_string()),
       //     granularity: RwSignal::new(Granularity::ThirtyMinute),
       //     aggregation: RwSignal::new(Aggregate::Avg),
            recipients: RwSignal::new(String::new()),
            schedule: RwSignal::new("*-*-* 01:00:00".to_string()),
            tags: RwSignal::new(String::new()),
        }
    }
}

impl ReportForm {
    
    pub fn update_from_report(&self, report: &Report) {
        self.name.set(report.name.clone());
        self.enabled.set(report.enabled);
        self.report_style.set(Some(report.report_style.clone()));
        self.days_back.set(report.days_back.to_string());
     //   self.granularity.set(report.granularity.clone());
     //   self.aggregation.set(report.aggregation.clone());
        self.recipients.set(report.recipients.join(","));
        self.schedule.set(report.schedule.clone());
        self.tags.set(report.tags.join(","));
    }

    // Convert ReportForm to Report
    pub fn to_report(&self) -> Report {
        Report {
            name: self.name.get().clone(),
            enabled: self.enabled.get(),
            report_style: self.report_style.get().clone().unwrap_or(String::from("report_style1")),
            site_group: self.site_group.get().as_ref().map_or(String::new(), |s| s.clone()),
            point_refs: self.point_refs.get().clone().unwrap_or_else(Vec::new),
            days_back: self.days_back.get().parse().unwrap_or_default(),
           // granularity: self.granularity.get().clone(),
           // aggregation: self.aggregation.get().clone(),
            recipients: self.recipients.get().split(',').map(|s| s.trim().to_string()).collect(),
            schedule: self.schedule.get().clone(),
            tags: self.tags.get().split(',').map(|s| s.trim().to_string()).collect(),
        }
    }
}


#[component]
pub fn ReportCreation(
                      #[prop(optional)] report: Option<Report>,
                     ) -> impl IntoView {

      
                      


    let report_form = ReportForm::default();
    let mut update = false;

    if let Some(report) = report {
        report_form.update_from_report(&report);
        update = true;
    }

    // let (form_submit_result, form_submit_result_set) = signal(Ok(""));

    let report_form_clone = report_form.clone();

    let form_dispatch_action =
        move || {
            let report = report_form_clone.to_report();

            // upload_report_action.dispatch(report);
        };

    view! {

        <ConfigProvider>

        <div>
            <div>
                <h2 class="text-2xl font-medium mb-4">Report Creation</h2>

                

                <div class="flex flex-wrap w-full px-4">
                    <div class="w-full md:w-1/2 p-2">
                        <div
                            class="border border-black-500 overflow-y-auto mt-4 flex-grow"
                            style="height: 600px;"
                        >
                            <Transition fallback=move || {
                                view! { <span>"Loading..."</span> }
                            }>

                             Something
                
                            </Transition>
                        </div>
                    </div>
                    <div class="w-full md:w-1/2 p-2">
                        <div
                            class="border border-black-500 overflow-y-auto mt-4 flex-grow"
                            style="height: 600px;"
                        >
                            <Transition fallback=move || {
                                view! { <span>"Loading..."</span> }
                            }>

                                
                           
                            <div>
        
        
                           
                            
        
                                <form
                                    class="w-full max-w-md mx-auto p-6"
                                    on:submit=|ev| ev.prevent_default()
                                >

        
                                    <div class="mb-4">
                                        <label
                                            for="report_style"
                                            class="block text-gray-700 font-bold mb-2"
                                        >
                                            Report Type
                                        </label>
        
                                        <ReportStyleCombobox report_style=report_form.report_style />
                                        <ReportStyleCombobox report_style=report_form.report_style />
                                        <ReportStyleCombobox report_style=report_form.report_style />
                                        <ReportStyleCombobox report_style=report_form.report_style />
                                        <ReportStyleCombobox report_style=report_form.report_style />
                                        <ReportStyleCombobox report_style=report_form.report_style />
        
                                    </div>
    
                                  
                                    <button
                                        type="submit"
                                        on:click=move |_| form_dispatch_action()
                                        class="w-full bg-blue-500 text-white py-2 px-4 rounded focus:outline-none focus:bg-blue-600 hover:bg-blue-600"
                                    >
                                        Submit
                                    </button>
        
                                    // the fallback receives a signal containing current errors
                                    <ErrorBoundary fallback=|_errors| {
                                        view! {
                                            <div class="error">
                                                <p>"Report creation failure: "</p>
                      
                                            </div>
                                        }
                                    }>
        
                                       //  <p>{form_submit_result}</p>
                                       <p></p>

                                    </ErrorBoundary>
        
                                </form>
        
                                <div style="z-index: 1;" class="flex flex-wrap gap-4"></div>
                            </div>
        
                     

                            </Transition>
                        </div>
                    </div>
                </div>

            </div>
        </div>

        </ConfigProvider>
        
    }

}

