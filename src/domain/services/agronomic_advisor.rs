//! # Oryza-Elo Architecture Guardrail: Agronomic Advisor Domain Service
//!
//! Trilingual agronomic advisory catalog providing operational field guidance
//! across all 7 phenological stages in Brazilian Portuguese (pt-BR), English (en), and Thai (th).

use crate::domain::models::advisory::FarmerAdvisory;
use crate::domain::models::locale::Locale;
use crate::domain::models::phenology::PhenologyStage;
use std::collections::BTreeMap;

/// Pure agronomic advisor providing static, immutable expert knowledge.
pub struct AgronomicAdvisor;

impl AgronomicAdvisor {
    /// Generates the advisory for a specific stage and target locale.
    pub fn generate_advisory(stage: PhenologyStage, locale: Locale) -> FarmerAdvisory {
        match (stage, locale) {
            // =========================================================================
            // 1. SEEDLING (Plântula / ระยะกล้า - BBCH 10-19)
            // =========================================================================
            (PhenologyStage::Seedling, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Fase de Plântula Estabelecida (BBCH 10–19)",
                "As plântulas de arroz estão desenvolvendo as primeiras folhas verdadeiras e sistema radicular inicial. O foco primordial é a uniformidade do estande e a proteção contra estresse hídrico e pragas iniciais de solo.",
                vec![
                    "Não permitir que a lâmina d'água cubra completamente as plântulas para evitar afogamento e morte por anoxia.".into(),
                    "Monitorar ataque inicial de lagarta-do-cartucho (Spodoptera frugiperda) e caramujos no leito de semeadura.".into(),
                ],
                vec![
                    "Manter solo saturado com lâmina capilar muito rasa (1–2 cm) para permitir respiração das raízes jovens.".into(),
                    "Aplicar adubação fosfatada e potássica de base de acordo com a análise de solo da ESALQ/USP.".into(),
                    "Realizar controle de plantas daninhas na fase de pré a pós-emergência precoce.".into(),
                ],
            ),
            (PhenologyStage::Seedling, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Seedling Establishment Stage (BBCH 10–19)",
                "Seedlings are developing primary root networks and initial true leaves. Priority is ensuring vigorous stand establishment, weed suppression, and avoiding submergence stress.",
                vec![
                    "Do not submerge seedlings under deep water; excessive flood depth leads to seedling anoxia and mortality.".into(),
                    "Scout for early seedling blight, thrips, and apple snails in newly flooded direct-seeded fields.".into(),
                ],
                vec![
                    "Maintain saturated soil or shallow flush (1–2 cm depth) to promote vigorous root elongation.".into(),
                    "Ensure basal phosphorus (P) and potassium (K) are incorporated in the root zone prior to permanent flood.".into(),
                    "Monitor weed emergence and prepare early post-emergence herbicide applications if required.".into(),
                ],
            ),
            (PhenologyStage::Seedling, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะกล้า (BBCH 10–19): การเจริญเติบโตของต้นกล้า",
                "ต้นข้าวอยู่ในช่วงสร้างรากและผลิใบจริงชุดแรก การจัดการเน้นที่ความสม่ำเสมอของแปลงต้นกล้า การคุมน้ำไม่ให้ท่วมยอด และการป้องกันโรคแมลงระยะเริ่มแรก",
                vec![
                    "ห้ามขังน้ำลึกจนท่วมยอดต้นกล้า เพราะจะทำให้ต้นกล้าเน่าและขาดออกซิเจนตาย".into(),
                    "เฝ้าระวังหอยเชอรี่และหนอนกระทู้คอรวงที่อาจกัดกินต้นกล้าอ่อน".into(),
                ],
                vec![
                    "รักษาระดับน้ำเพียง 1–2 เซนติเมตร หรือดินเปียกสลับแห้งเพื่อให้รากหยั่งลึก".into(),
                    "ใส่ปุ๋ยรองพื้นสูตรที่มีฟอสฟอรัสสูง เช่น 16-20-0 ตามคำแนะนำของกรมการข้าว".into(),
                    "กำจัดวัชพืชในแปลงก่อนที่วัชพืชจะแย่งแสงแดดและธาตุอาหาร".into(),
                ],
            ),

            // =========================================================================
            // 2. TILLERING (Perfilhamento / แตกกอ - BBCH 20-29)
            // =========================================================================
            (PhenologyStage::Tillering, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Fase de Perfilhamento Ativo (BBCH 20–29)",
                "A lavoura está emitindo afilhos secundários e terciários, definindo o número potencial de panículas por metro quadrado. A nutrição nitrogenada e a lâmina controlada são decisivas.",
                vec![
                    "Evitar estresse hídrico severo; o perfilhamento cessa imediatamente sob déficit de umidade.".into(),
                    "Não elevar a lâmina acima de 7 cm, pois excesso de profundidade inibe o surgimento de novos afilhos férteis.".into(),
                ],
                vec![
                    "Manter lâmina de irrigação rasa e constante de 2 a 5 cm para acelerar a brotação de colmos.".into(),
                    "Aplicar primeira fração de nitrogênio em cobertura (ureia ou sulfato de amônio) no início do perfilhamento ativo.".into(),
                    "Vistoriar a base dos colmos para detectar bicho-do-coração e sintomas iniciais de mancha-bainha.".into(),
                ],
            ),
            (PhenologyStage::Tillering, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Active Tillering Stage (BBCH 20–29)",
                "Tillers are multiplying rapidly, establishing the panicle-bearing capacity of the crop. Moisture availability and nitrogen timing dictate productive tiller percentage.",
                vec![
                    "Do not allow drought stress; tillering halts quickly when soil moisture drops below saturation.".into(),
                    "Avoid deep standing water (> 7 cm) as hydrostatic pressure restricts lower node tiller bud development.".into(),
                ],
                vec![
                    "Maintain a shallow continuous water layer of 2–5 cm to stimulate tillering nodes.".into(),
                    "Apply the first split of nitrogen fertilizer onto muddy soil right before flooding to maximize uptake efficiency.".into(),
                    "Scout for stem borer 'dead heart' damage and sheath blight lesions near the waterline.".into(),
                ],
            ),
            (PhenologyStage::Tillering, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะแตกกอ (BBCH 20–29): การสร้างหน่อและรวงผลผลิต",
                "ต้นข้าวกำลังแตกหน่อและสร้างหน่อใหม่ ซึ่งเป็นตัวกำหนดจำนวนรวงต่อตารางเมตร จำเป็นต้องจัดการน้ำและปุ๋ยไนโตรเจนอย่างเหมาะสม",
                vec![
                    "ระวังแปลงขาดน้ำ การขาดน้ำในระยะนี้จะทำให้ข้าวหยุดแตกกอทันที".into(),
                    "อย่าขังน้ำลึกเกิน 7 เซนติเมตร เพราะระดับน้ำสูงจะกดทับข้อตาไม่ให้แตกหน่อใหม่".into(),
                ],
                vec![
                    "รักษาระดับน้ำตื้น 2–5 เซนติเมตร สม่ำเสมอเพื่อส่งเสริมการแตกกอที่สมบูรณ์".into(),
                    "หว่านปุ๋ยไนโตรเจนแต่งหน้าครั้งแรก (เช่น ยูเรีย 46-0-0 ร่วมกับ 16-20-0) ขณะดินมีความชื้น".into(),
                    "ตรวจแปลงเพื่อเฝ้าระวังหนอนกอข้าว (อาการยอดเหี่ยว) และโรคกาบใบแห้ง".into(),
                ],
            ),

            // =========================================================================
            // 3. BOOTING (Emborrachamento / ตั้งท้อง - BBCH 40-49)
            // =========================================================================
            (PhenologyStage::Booting, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Fase Crítica de Emborrachamento (BBCH 40–49)",
                "A panícula jovem está se desenvolvendo dentro da bainha da folha-bandeira. Trata-se da fase de maior suscetibilidade a deficiências nutricionais e estresses térmicos.",
                vec![
                    "JANELA CRÍTICA: Não aplicar herbicidas pós-tardios, risco de deformação de panícula e abortamento floral.".into(),
                    "Monitorar rigorosamente brusone nas folhas (Magnaporthe oryzae); infecções precoces invadem o nó da panícula.".into(),
                ],
                vec![
                    "Aplicar adubação de cobertura final de nitrogênio e potássio (topdressing) para maximizar grãos por panícula.".into(),
                    "Manter lâmina de água de 5 a 10 cm; a água atua como regulador térmico contra noites frias (< 15°C) ou picos de calor (> 35°C).".into(),
                    "Inspecionar a expansão da folha-bandeira e a saúde fitossanitária de todas as folhas superiores.".into(),
                ],
            ),
            (PhenologyStage::Booting, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Critical Booting Stage (BBCH 40–49)",
                "The young panicle is differentiating and swelling inside the flag leaf sheath. High physiological sensitivity requires optimal nutrition and thermal buffering.",
                vec![
                    "CRITICAL WINDOW: Avoid post-booting herbicide applications to prevent severe panicle deformation and spikelet sterility.".into(),
                    "Scout proactively for leaf blast (Magnaporthe oryzae); undetected lesions will compromise the panicle neck.".into(),
                ],
                vec![
                    "Apply panicle initiation nitrogen topdress (and potassium) to maximize spikelet count and grain filling capacity.".into(),
                    "Maintain adequate flood depth (5–10 cm) to buffer microclimate temperatures against cold dips or extreme heat.".into(),
                    "Inspect the flag leaf collar and prepare preventive fungicide applications if fungal pressure is high.".into(),
                ],
            ),
            (PhenologyStage::Booting, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะตั้งท้อง (BBCH 40–49): ช่วงเวลาวิกฤตของการสร้างรวงข้าว",
                "รวงข้าวอ่อนกำลังเจริญเติบโตอยู่ภายในกาบใบธง เป็นช่วงที่มีความอ่อนไหวสูงต่อสภาพอากาศและต้องการสารอาหารสูงสุดเพื่อกำหนดจำนวนเมล็ดต่อรวง",
                vec![
                    "ข้อควรระวังสำคัญ: ห้ามฉีดพ่นสารกำจัดวัชพืชในระยะนี้โดยเด็ดขาด เพราะจะทำให้รวงข้าวบิดเบี้ยวและเมล็ดลีบ".into(),
                    "เฝ้าระวังโรคไหม้ (Magnaporthe oryzae) อย่างเข้มงวด หากระบาดจะลามไปทำลายคอรวง".into(),
                ],
                vec![
                    "หว่านปุ๋ยรับรวง / ปุ๋ยแต่งหน้า (ไนโตรเจนและโพแทสเซียม) ในช่วงกำเนิดช่อดอกเพื่อเพิ่มจำนวนเมล็ดต่อรวง".into(),
                    "รักษาระดับน้ำในแปลง 5–10 เซนติเมตร เพื่อรักษาอุณหภูมิแปลงไม่ให้แกว่งเกินไป".into(),
                    "สำรวจใบธงเพื่อป้องกันแมลงศัตรูพืชและโรคทางใบก่อนข้าวโพล่รวง".into(),
                ],
            ),

            // =========================================================================
            // 4. HEADING (Espigamento / ออกรวง - BBCH 50-59)
            // =========================================================================
            (PhenologyStage::Heading, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Fase de Espigamento / Emissão das Panículas (BBCH 50–59)",
                "As panículas estão emergindo das bainhas. Fase de transição rápida que antecede imediatamente a antese, exigindo monitoramento rigoroso de pragas sugadoras de grãos.",
                vec![
                    "Não permitir drenagem do talhão; a falta de água no espigamento compromete o esticamento completo da panícula.".into(),
                    "Monitorar infestação de percevejo-do-arroz (Oebalus poecilus) que pica os grãos no início da emissão.".into(),
                ],
                vec![
                    "Garantir suprimento hídrico pleno com lâmina contínua de 5 a 7 cm.".into(),
                    "Efetuar vistoria no nó da panícula para controle preventivo de brusone-do-pescoço.".into(),
                    "Acompanhar previsões meteorológicas para ocorrência de ventos secos ou chuvas contínuas.".into(),
                ],
            ),
            (PhenologyStage::Heading, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Panicle Exsertion / Heading Stage (BBCH 50–59)",
                "Panicles are rapidly emerging from the flag leaf sheaths. This stage sets up the flowering window and requires vigilant protection against panicle pests.",
                vec![
                    "Do not drain the field; water deficit during heading leads to incomplete panicle exsertion and trapped spikelets.".into(),
                    "Scout for rice stink bugs (Oebalus spp.) and armyworms attacking emerging panicle structures.".into(),
                ],
                vec![
                    "Maintain continuous water depth of 5–7 cm to prevent moisture stress.".into(),
                    "Apply protective neck blast treatments if disease history and weather conditions warrant.".into(),
                    "Monitor wind patterns and relative humidity trends ahead of anthesis.".into(),
                ],
            ),
            (PhenologyStage::Heading, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะออกรวง (BBCH 50–59): รวงข้าวโพล่ออกจากกาบใบธง",
                "รวงข้าวกำลังโพล่ออกมาอย่างสมบูรณ์ เป็นช่วงต่อสู่การผสมเกสร ต้องรักษาความชื้นและระวังศัตรูพืชที่เข้าทำลายรวง",
                vec![
                    "ห้ามปล่อยให้แปลงแห้งเด็ดขาด การขาดน้ำจะทำให้รวงข้าวโพล่ไม่พ้นกาบใบและเมล็ดติดไม่เต็มรวง".into(),
                    "เฝ้าระวังมวนสิงและเพลี้ยไฟที่เข้าดูดกินน้ำเลี้ยงที่รวงข้าวที่เพิ่งโพล่".into(),
                ],
                vec![
                    "รักษาระดับน้ำในแปลงไว้ที่ 5–7 เซนติเมตร อย่างสม่ำเสมอ".into(),
                    "ตรวจสอบอาการโรคเน่าคอรวงและโรคใบขีดสีน้ำตาล".into(),
                    "เตรียมพร้อมรับมือสภาพอากาศ เช่น ฝนตกชุกหรือลมกระโชกแรง".into(),
                ],
            ),

            // =========================================================================
            // 5. FLOWERING (Floração / ออกดอก - BBCH 60-69)
            // =========================================================================
            (PhenologyStage::Flowering, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Floração e Antese Ativa (BBCH 60–69): Manejo Crítico",
                "As flores do arroz estão abertas realizando a polinização e fecundação dos óvulos. Qualquer distúrbio físico, químico ou hídrico nesta fase causa esterilidade permanente das espiguetas.",
                vec![
                    "RESTRIÇÃO OPERACIONAL MANDATÓRIA: PROIBIDA A PULVERIZAÇÃO DE DEFENSIVOS DAS 09:00 ÀS 12:00! A aplicação no período de abertura das anteras desseca o grão de pólen, queima os estigmas e gera esterilidade/abortamento floral severo.".into(),
                    "Temperaturas acima de 35°C ou vento ressecante durante a manhã induzem alta esterilidade floral.".into(),
                ],
                vec![
                    "Se aplicações fitossanitárias forem inadiáveis, pulverizar exclusivamente no final da tarde (após as 16:30) ou à noite.".into(),
                    "Manter lâmina de água de 5 a 10 cm como amortecedor microclimático para manter a umidade relativa do dossel acima de 70%.".into(),
                    "Evitar trânsito de máquinas pesadas ou pessoas no interior do talhão durante a manhã.".into(),
                ],
            ),
            (PhenologyStage::Flowering, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Anthesis & Flowering Stage (BBCH 60–69): Critical Window",
                "Spikelets are actively opening, releasing pollen, and undergoing fertilization. The crop is at its peak vulnerability to chemical damage and microclimatic shocks.",
                vec![
                    "MANDATORY OPERATIONAL RESTRICTION: STRICT PROHIBITION OF CHEMICAL SPRAYING BETWEEN 09:00 AND 12:00! Agrochemical applications during morning anther dehiscence desiccate pollen grains, destroy stigma receptivity, and result in severe floret sterility and blank grains.".into(),
                    "High temperatures exceeding 35°C or strong dry winds during anthesis induce acute floret abortion.".into(),
                ],
                vec![
                    "Schedule unavoidable fungicide or insecticide applications strictly for the late afternoon (after 16:30) or night.".into(),
                    "Maintain continuous standing water (5–10 cm) to elevate canopy relative humidity and buffer midday heat extremes.".into(),
                    "Keep tractor and worker field traffic away during peak morning anthesis hours.".into(),
                ],
            ),
            (PhenologyStage::Flowering, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะออกดอกและผสมเกสร (BBCH 60–69): ข้อจำกัดวิกฤตสำหรับการจัดการแปลง",
                "ดอกข้าวกำลังบานและผสมเกสร เกสรตัวผู้กำลังเปิดรับการปฏิสนธิ เป็นช่วงที่มีความเปราะบางสูงสุดต่อสารเคมีและความร้อน",
                vec![
                    "ข้อห้ามเด็ดขาดด้านการปฏิบัติการ: ห้ามฉีดพ่นสารเคมีทุกชนิดในช่วงเวลา 09:00 ถึง 12:00 น. โดยเด็ดขาด! การพ่นยาขณะเกสรข้าวกำลังบานจะล้างละอองเกสรและทำให้เกสรแห้งตาย ส่งผลให้ข้าวเป็นเมล็ดลีบทั้งแปลง".into(),
                    "อากาศร้อนจัดเกิน 35°C หรือลมพัดแรงในช่วงเช้าจะเพิ่มความเสี่ยงต่อการผสมไม่ติด".into(),
                ],
                vec![
                    "หากจำเป็นต้องฉีดพ่นยาป้องกันกำจัดศัตรูพืช ให้ทำเฉพาะช่วงเย็นหลังเวลา 16:30 น. เป็นต้นไป".into(),
                    "รักษาระดับน้ำในแปลง 5–10 เซนติเมตร เพื่อเพิ่มความชื้นสัมพัทธ์ในแปลงและลดความร้อนรอบช่อดอก".into(),
                    "หลีกเลี่ยงการเดินลุยแปลงข้าวในช่วงเช้าขณะที่ข้าวกำลังผสมเกสร".into(),
                ],
            ),

            // =========================================================================
            // 6. PRE-HARVEST (Maturação Pré-Colheita / ก่อนเก็บเกี่ยว - BBCH 70-89)
            // =========================================================================
            (PhenologyStage::PreHarvest, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Maturação Pré-Colheita / Enchimento de Grãos (BBCH 70–89)",
                "Os grãos passaram pelo estádio leitoso e pastoso, atingindo a maturação vítrea. As panículas curvam-se em tom dourado com mais de 80% dos grãos amadurecidos.",
                vec![
                    "Drenar o talhão muito cedo reduz o enchimento dos grãos do terço inferior da panícula.".into(),
                    "Evitar atraso na drenagem em solos argilosos, o que impediria a entrada da colhedora.".into(),
                ],
                vec![
                    "Iniciar drenagem gradual do talhão entre 10 a 14 dias antes da data estimada de colheita para firmar o piso da máquina.".into(),
                    "Monitorar teor de umidade dos grãos no campo com medidor portátil (faixa ideal para corte: 20% a 23%).".into(),
                    "Revisar maquinário, plataformas de corte e estrutura de transporte para evitar perdas pós-colheita.".into(),
                ],
            ),
            (PhenologyStage::PreHarvest, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Pre-Harvest Ripening & Dough Stage (BBCH 70–89)",
                "Grains have transitioned through milk and dough stages to hard grain. Panicles are curving downward and turning golden yellow with over 80% matured spikelets.",
                vec![
                    "Draining the field prematurely halts carbohydrate translocation to lower panicle spikelets, increasing chalky grains.".into(),
                    "Late drainage in heavy clay soils will cause harvesting equipment to bog down and compact the subsoil.".into(),
                ],
                vec![
                    "Commence terminal field drainage 10–14 days prior to expected harvest to firm soil bearing capacity.".into(),
                    "Sample grain moisture content across multiple plot points (ideal harvest moisture: 20–23%).".into(),
                    "Calibrate combine harvesters and inspect drying silos for incoming grain batches.".into(),
                ],
            ),
            (PhenologyStage::PreHarvest, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะก่อนเก็บเกี่ยว (BBCH 70–89): ข้าวสุกแก่และสะสมแป้ง",
                "เมล็ดข้าวเปลี่ยนจากระยะน้ำนมเป็นแป้งแข็ง รวงข้าวเริ่มโค้งโน้มลง เมล็ดเริ่มเปลี่ยนเป็นสีเหลืองฟางมากกว่า 80% ของรวง",
                vec![
                    "การระบายน้ำออกเร็วเกินไปจะทำให้เมล็ดโคนรวงลีบ แป้งไม่เต็มเมล็ด".into(),
                    "การระบายน้ำช้าเกินไปในดินเหนียวจะทำให้รถเกี่ยวข้าวติดหล่มและเก็บเกี่ยวล่าช้า".into(),
                ],
                vec![
                    "ระบายน้ำออกจากแปลงก่อนการเก็บเกี่ยวประมาณ 10–14 วัน เพื่อให้ผิวดินแห้งแข็งรองรับรถเกี่ยว".into(),
                    "ตรวจวัดความชื้นของเมล็ดข้าวในแปลง (ความชื้นที่เหมาะสมสำหรับการเก็บเกี่ยวคือ 20–23%)".into(),
                    "จัดเตรียมรถเกี่ยวข้าว ลานตาก และโรงเก็บข้าวให้พร้อมล่วงหน้า".into(),
                ],
            ),

            // =========================================================================
            // 7. HARVEST-READY (Colheita Plena / ก่อนเก็บเกี่ยวเกี่ยว - BBCH 90-99)
            // =========================================================================
            (PhenologyStage::HarvestReady, Locale::PtBr) => FarmerAdvisory::new(
                Locale::PtBr,
                stage,
                "Colheita Plena / Ponto Ótimo de Corte (BBCH 90–99)",
                "A lavoura atingiu a maturidade fisiológica completa com mais de 90% dos grãos duros e palha seca. Ponto ideal para colheita mecânica.",
                vec![
                    "Atrasar a colheita com umidade abaixo de 16% causa trincas no grão durante o beneficiamento e debulha natural no campo.".into(),
                    "Colheita sob chuva causa fermentação rápida nos grãos colhidos a granel; secagem deve ocorrer em até 24 horas.".into(),
                ],
                vec![
                    "Operar colhedoras com velocidade adequada para minimizar perdas na barra de corte.".into(),
                    "Encaminhar imediatamente a carga para secagem artificial lenta até 13–14% de umidade padrão.".into(),
                    "Planejar rotação de culturas ou dessecação de soqueira logo após o término da colheita.".into(),
                ],
            ),
            (PhenologyStage::HarvestReady, Locale::En) => FarmerAdvisory::new(
                Locale::En,
                stage,
                "Harvest-Ready / Full Maturity (BBCH 90–99)",
                "The crop has attained full physiological maturity with over 90% of grains hardened and straw senescing. Optimal window for combine harvesting.",
                vec![
                    "Over-ripening in the field below 16% moisture leads to severe grain fissuring, sun-checking, and head rice yield loss.".into(),
                    "Grains harvested wet must be transferred to drying facilities within 24 hours to prevent mycotoxin and heating decay.".into(),
                ],
                vec![
                    "Execute combine operations with proper cylinder speed and clearance settings to minimize threshing cracked grains.".into(),
                    "Dry harvested paddy steadily down to the safe storage equilibrium of 13–14% moisture content.".into(),
                    "Plan residue incorporation or cover cropping immediately following parcel clearance.".into(),
                ],
            ),
            (PhenologyStage::HarvestReady, Locale::Th) => FarmerAdvisory::new(
                Locale::Th,
                stage,
                "ระยะเก็บเกี่ยวเต็มที่ (BBCH 90–99): ข้าวสุกแก่พร้อมเก็บเกี่ยว",
                "ข้าวสุกแก่เต็มที่พร้อมเก็บเกี่ยว เมล็ดข้าวกว่า 90% เปลี่ยนเป็นสีเหลืองทองและแข็งตัว ฟางข้าวเริ่มแห้งกรอบ ได้เวลาเก็บเกี่ยวผลผลิต",
                vec![
                    "อย่าปล่อยให้ข้าวสุกค้างแปลงจนแห้งเกินไป (ความชื้นต่ำกว่า 16%) เมล็ดจะแตกร้าวเมื่อนำไปสีและร่วงหล่นในแปลงสูง".into(),
                    "หากเก็บเกี่ยวขณะเมล็ดมีความชื้นสูง ต้องนำข้าวไปอบหรือตากลดความชื้นภายใน 24 ชั่วโมงเพื่อป้องกันเชื้อรา".into(),
                ],
                vec![
                    "ปรับตั้งรอบเครื่องและตะแกรงรถเกี่ยวข้าวให้เหมาะสมเพื่อลดการสูญเสียและการกะเทาะเมล็ด".into(),
                    "ลดความชื้นข้าวเปลือกให้เหลือ 14% เพื่อความปลอดภัยในการเก็บรักษาในยุ้งฉาง".into(),
                    "เตรียมจัดการตอซังข้าวโดยการหมักย่อยสลาย หลีกเลี่ยงการเผาตอซังเพื่อลดมลพิษ PM 2.5".into(),
                ],
            ),
        }
    }

    /// Generates advisories for all 3 supported locales at once, keyed by their ISO code string
    /// ("pt-BR", "en", "th"). This enables edge clients (e.g. Flutter Web) to store all translations locally
    /// for instant offline language switching without calling the server.
    pub fn generate_all_translations(stage: PhenologyStage) -> BTreeMap<String, FarmerAdvisory> {
        let mut map = BTreeMap::new();
        for &locale in Locale::all() {
            let advisory = Self::generate_advisory(stage, locale);
            map.insert(locale.as_str().to_string(), advisory);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_stages_have_complete_advisories() {
        for &stage in &PhenologyStage::ALL_CHRONOLOGICAL {
            for &locale in Locale::all() {
                let adv = AgronomicAdvisor::generate_advisory(stage, locale);
                assert_eq!(adv.stage, stage);
                assert_eq!(adv.locale, locale);
                assert!(!adv.headline.trim().is_empty(), "Headline missing for {:?} in {:?}", stage, locale);
                assert!(!adv.plain_text.trim().is_empty(), "Plain text missing for {:?} in {:?}", stage, locale);
                assert!(!adv.urgent_warnings.is_empty(), "Urgent warnings missing for {:?} in {:?}", stage, locale);
                assert!(!adv.management_tips.is_empty(), "Management tips missing for {:?} in {:?}", stage, locale);
            }
        }
    }

    #[test]
    fn test_flowering_contains_mandatory_morning_spray_warning() {
        for &locale in Locale::all() {
            let adv = AgronomicAdvisor::generate_advisory(PhenologyStage::Flowering, locale);
            let combined_warnings = adv.urgent_warnings.join(" ");

            match locale {
                Locale::PtBr => {
                    assert!(
                        combined_warnings.contains("09:00") && combined_warnings.contains("12:00") && combined_warnings.contains("PROIBIDA"),
                        "Portuguese flowering advisory missing morning spray restriction: {}", combined_warnings
                    );
                }
                Locale::En => {
                    assert!(
                        combined_warnings.contains("09:00") && combined_warnings.contains("12:00") && combined_warnings.contains("PROHIBITION"),
                        "English flowering advisory missing morning spray restriction: {}", combined_warnings
                    );
                }
                Locale::Th => {
                    assert!(
                        combined_warnings.contains("09:00") && combined_warnings.contains("12:00") && combined_warnings.contains("ห้าม"),
                        "Thai flowering advisory missing morning spray restriction: {}", combined_warnings
                    );
                }
            }
        }
    }

    #[test]
    fn test_booting_contains_blast_and_nitrogen_guidance() {
        let adv_pt = AgronomicAdvisor::generate_advisory(PhenologyStage::Booting, Locale::PtBr);
        assert!(adv_pt.urgent_warnings.iter().any(|w| w.contains("brusone") || w.contains("Magnaporthe")));
        assert!(adv_pt.management_tips.iter().any(|t| t.contains("nitrogênio") || t.contains("topdressing")));

        let adv_en = AgronomicAdvisor::generate_advisory(PhenologyStage::Booting, Locale::En);
        assert!(adv_en.urgent_warnings.iter().any(|w| w.contains("blast") || w.contains("Magnaporthe")));
        assert!(adv_en.management_tips.iter().any(|t| t.contains("nitrogen") || t.contains("topdress")));

        let adv_th = AgronomicAdvisor::generate_advisory(PhenologyStage::Booting, Locale::Th);
        assert!(adv_th.urgent_warnings.iter().any(|w| w.contains("โรคไหม้") || w.contains("Magnaporthe")));
        assert!(adv_th.management_tips.iter().any(|t| t.contains("ปุ๋ยรับรวง") || t.contains("ปุ๋ยแต่งหน้า")));
    }

    #[test]
    fn test_generate_all_translations() {
        let translations = AgronomicAdvisor::generate_all_translations(PhenologyStage::Heading);
        assert_eq!(translations.len(), 3);
        assert!(translations.contains_key("pt-BR"));
        assert!(translations.contains_key("en"));
        assert!(translations.contains_key("th"));
    }
}
