pub fn check_surface(
    hab_super: String,
    lir_super: String,
    kit_super: String,
    sbat_super: String,
    sbed_super: String,
    nlr: String,
    nk: String,
    nbat: String,
    nt: String,
    nbed: String,
    nfr: String,
) -> bool {
    let global_super = hab_super.parse::<f64>().unwrap_or(0.0);
    let living_super = lir_super.parse::<f64>().unwrap_or(0.0);
    let kitchen_super = kit_super.parse::<f64>().unwrap_or(0.0);
    let bath_super = sbat_super.parse::<f64>().unwrap_or(0.0);
    let bed_super = sbed_super.parse::<f64>().unwrap_or(0.0);
    let living_number = nlr.parse::<f64>().unwrap_or(0.0);
    let kitchen_number = nk.parse::<f64>().unwrap_or(0.0);
    let bath_number = nbat.parse::<f64>().unwrap_or(0.0);
    let bed_number = nbed.parse::<f64>().unwrap_or(0.0);
    let wc_number = nt.parse::<f64>().unwrap_or(0.0);
    let free_number = nfr.parse::<f64>().unwrap_or(0.0);

    // println!(
    //     "global_super : {} | living_super : {} | living_number : {} | kitchen_super : {} | kitchen_number : {} | bath_super : {} | bath_number : {} | bed_super : {} | bed_number : {} | wc_number : {} | free_number : {} | result : {}",
    //     global_super,
    //     living_super,
    //     living_number,
    //     kitchen_super,
    //     kitchen_number,
    //     bath_super,
    //     bath_number,
    //     bed_super,
    //     bed_number,
    //     wc_number,
    //     free_number,
    //     global_super
    //         >= living_super * living_number
    //             + kitchen_super * kitchen_number
    //             + bath_super * bath_number
    //             + bed_super * bed_number
    //             + wc_number * 2.0
    //             + free_number * 2.0
    // );

    global_super
        >= living_super * living_number
            + kitchen_super * kitchen_number
            + bath_super * bath_number
            + bed_super * bed_number
            + wc_number * 2.0
            + free_number * 2.0
}
