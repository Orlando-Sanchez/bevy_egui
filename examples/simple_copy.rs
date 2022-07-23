use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{RichText, Color32, plot::{BarChart, Bar}, Widget};
use std::f64::consts::TAU;

#[derive(Default)]
struct UiState {
    label: String,
    account_create: bool,
    account_seeded: bool,
    account_fund: bool,
    account_exists: bool,
    account_balance: bool,
    asset_create_class: bool,
    asset_class_info: bool,
    asset_create: bool,
    asset_info: bool,
    asset_update_metadata: bool,
    asset_mint: bool,
    asset_burn: bool,
    asset_balance: bool,
    asset_transfer_from: bool,
    bag_register: bool,
    bag_create: bool,
    bag_sweep: bool,
    bag_deposit: bool,
    bundle_register: bool,
    bundle_mint: bool,
    bundle_burn: bool,
    market_create: bool,
    market_create_market_rate: bool,
    market_deposit_assets: bool,
    market_exchange_assets: bool
}

impl Widget for &mut YourThing {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>,) {
    // egui::SidePanel::left("left_side_panel")
    // .default_width(200.0)
    // .show(egui_context.ctx_mut(), |ui| {
    //     egui::menu::bar(ui, |ui| {
    //         ui.vertical(|ui| {
    //             egui::menu::menu_button(ui, "Account", |ui| {
    //                 egui::menu::bar(ui, |ui| {
    //                     ui.vertical(|ui| {
    //                         if ui.button("Create account").clicked() {
    //                             ui_state.account_create = true
    //                         }

    //                         ui.separator();
                            
    //                         if ui.button("Account seeded").clicked() {
    //                             ui_state.account_seeded = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Fund account").clicked() {
    //                             ui_state.account_fund = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Account exists").clicked() {
    //                             ui_state.account_exists = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Account balance").clicked() {
    //                             ui_state.account_balance = true
    //                         }

    //                     })
    //                 });
    //             });

    //             ui.separator();

    //             egui::menu::menu_button(ui, "Asset", |ui| {
    //                 egui::menu::bar(ui, |ui| {
    //                     ui.vertical(|ui| {
    //                         if ui.button("Create asset class").clicked() {
    //                             ui_state.asset_create_class = true
    //                         }

    //                         ui.separator();
                            
    //                         if ui.button("Asset class info").clicked() {
    //                             ui_state.asset_class_info = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Create asset").clicked() {
    //                             ui_state.asset_create = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Asset info").clicked() {
    //                             ui_state.asset_info = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Update asset metadata").clicked() {
    //                             ui_state.asset_update_metadata = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Mint asset").clicked() {
    //                             ui_state.asset_mint = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Burn asset").clicked() {
    //                             ui_state.asset_burn = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Asset balance").clicked() {
    //                             ui_state.asset_balance = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Transfer asset").clicked() {
    //                             ui_state.asset_transfer_from = true
    //                         }
    //                     })
    //                 });
    //             });

    //             ui.separator();

    //             egui::menu::menu_button(ui, "Bag", |ui| {
    //                 egui::menu::bar(ui, |ui| {
    //                     ui.vertical(|ui| {
    //                         if ui.button("Register bag").clicked() {
    //                             ui_state.bag_register= true
    //                         }

    //                         ui.separator();
                            
    //                         if ui.button("Create bag").clicked() {
    //                             ui_state.bag_create = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Deposit bag").clicked() {
    //                             ui_state.bag_deposit = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Sweep bag").clicked() {
    //                             ui_state.bag_sweep = true
    //                         }
    //                     })
    //                 });
    //             });

    //             ui.separator();

    //             egui::menu::menu_button(ui, "Bundle", |ui| {
    //                 egui::menu::bar(ui, |ui| {
    //                     ui.vertical(|ui| {
    //                         if ui.button("Register bundle").clicked() {
    //                             ui_state.bundle_register = true
    //                         }

    //                         ui.separator();
                            
    //                         if ui.button("Mint bundle").clicked() {
    //                             ui_state.bundle_mint = true
    //                         }

    //                         ui.separator();

    //                         if ui.button("Burn bundle").clicked() {
    //                             ui_state.bundle_burn = true
    //                         }
    //                     })
    //                 });
    //             });

    //             ui.separator();

    //             egui::menu::menu_button(ui, "Market", |ui| {
    //                 egui::menu::bar(ui, |ui| {
    //                     ui.vertical(|ui| {
    //                         if ui.button("Create market").clicked() {
    //                             ui_state.market_create = true
    //                         }

    //                         ui.separator();
                            
    //                         if ui.button("Create market rate").clicked() {
    //                             ui_state.market_create_market_rate = true
    //                         }

    //                         ui.separator();
                        
    //                         if ui.button("Deposit assets").clicked() {
    //                             ui_state.market_deposit_assets = true
    //                         }

    //                         ui.separator();
                            
    //                         if ui.button("Exchange assets").clicked() {
    //                             ui_state.market_exchange_assets = true
    //                         } 
    //                     })
    //                 });
    //             });
    //         });
    //     })
    // });

    // if ui_state.account_create {
    //     egui::Window::new("Create account").show(egui_context.ctx_mut(), |ui| {
    //         ui.label("Account created!");
    //     });
    // }

    // if ui_state.account_seeded {
    //     egui::Window::new("Account seeded").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });
    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.account_fund {
    //     egui::Window::new("Fund account").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amount: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("To: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.account_exists {
    //     egui::Window::new("Account exists").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Account: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.account_balance {
    //     egui::Window::new("Account balance").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Account: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_create_class {
    //     egui::Window::new("Create asset class").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Metadata: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Owner: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_class_info {
    //     egui::Window::new("Asset class info").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_create {
    //     egui::Window::new("Create asset").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });
                
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Account: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });  

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Metadata: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_info {
    //     egui::Window::new("Asset info").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_update_metadata {
    //     egui::Window::new("Update asset metadata").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Metadata: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_mint {
    //     egui::Window::new("Mint asset").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("To: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amount: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_burn {
    //     egui::Window::new("Burn asset").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("From: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amount: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_balance {
    //     egui::Window::new("Asset balance").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Account: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.asset_transfer_from {
    //     egui::Window::new("Transfer asset").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("From: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("To: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amount: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bag_register {
    //     egui::Window::new("Register bag").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Metadata: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bag_create {
    //     egui::Window::new("Create bag").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Owners: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Shares: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bag_deposit {
    //     egui::Window::new("Deposit bag").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Bag: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class ids: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset ids: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amounts: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bag_sweep {
    //     egui::Window::new("Sweep bag").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("To: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Bag: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset ids: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amounts: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bundle_register {
    //     egui::Window::new("Register bundle").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Metadata: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Schema: ");
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Class ids: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Asset ids: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amounts: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bundle_mint {
    //     egui::Window::new("Mint bundle").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("From: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("To: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Bundle id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amount: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.bundle_burn {
    //     egui::Window::new("Burn bundle").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("From: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("To: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Bundle id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Amount: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    // if ui_state.market_create {
    //     egui::Window::new("Create market").show(egui_context.ctx_mut(), |ui| {
    //         ui.vertical(|ui| {
    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Seed: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(6.0);
    
    //                 ui.vertical(|ui| {
    //                     ui.add_space(15.0);
    //                     ui.label("Market id: ");
    //                 });
    //                 ui.vertical(|ui| {
    //                     ui.add_space(12.0);
    //                     ui.text_edit_singleline(&mut ui_state.label);
    //                 });
    //             });

    //             ui.horizontal(|ui| {
    //                 ui.add_space(285.0);
    //                 if ui.button(RichText::new("Submit").color(Color32::RED)).clicked() {
                        
    //                 }

    //             })
    //         });
    //     });
    // }

    let mut chart1 = BarChart::new(vec![
        Bar::new(0.5, 1.0).name("Day 1"),
        Bar::new(1.5, 3.0).name("Day 2"),
        Bar::new(2.5, 1.0).name("Day 3"),
        Bar::new(3.5, 2.0).name("Day 4"),
        Bar::new(4.5, 4.0).name("Day 5"),
    ])
    .width(0.7)
    .name("Set 1");
    
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| { 
        // let pos_radius = 8.0;
        // let tip_radius = 7.0;
        // let arrow_origins = egui::widgets::plot::Values::from_parametric_callback(
        //     |t| (pos_radius * t.sin(), pos_radius * t.cos()),
        //     0.0..TAU,
        //     36,
        // );
        // let arrow_tips = egui::widgets::plot::Values::from_parametric_callback(
        //     |t| (tip_radius * t.sin(), tip_radius * t.cos()),
        //     0.0..TAU,
        //     36,
        // );
        // ui.add(egui::plot::Arrows::new(arrow_origins, arrow_tips));
        // ui.add(egui::Slider::new(&mut 3.3, 0.0..=100.0).text("My value"));

        ui.add(chart1)

    });

}
