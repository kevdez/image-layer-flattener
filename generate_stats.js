import fs from "fs";
import path from "path";

const folder = "./results/nft-metadata";
const instructionsFile = "./image_map_with_classes.json";

const stats = {
  ClassStats: {
    Calaverita: {},
    Catrin: {},
    Catrina: {},
    Especial: {}
  },
  Calaveritas: []
};

const instructionsPath = path.resolve(__dirname, instructionsFile);
const instructionsBuffer = fs.readFileSync(instructionsPath);

const instructions = JSON.parse(Buffer.from(instructionsBuffer));

instructions.classes.forEach(({ class_name, layers }) => {
  layers.forEach(({ trait_type, distribution }) => {
    distribution.forEach(({ value, omit_from_metadata }) => {
      if (!omit_from_metadata) {
        stats.ClassStats[class_name][trait_type] = {
          ...stats.ClassStats[class_name][trait_type]
        };
        stats.ClassStats[class_name][trait_type][value] = 0;
      }
    });
  });
});

const getClassTraitType = (trait) => trait.trait_type === "Class";

fs.readdirSync(folder).forEach((file) => {
  const filePath = path.resolve(__dirname, folder, file);
  const openFile = fs.readFileSync(filePath);

  const metadataObj = JSON.parse(Buffer.from(openFile));

  stats.Calaveritas.push(metadataObj);

  const { value: calaveritaClass } = metadataObj.attributes.find(getClassTraitType);
  metadataObj.attributes.forEach(({ trait_type, value }) => {
    if (stats.ClassStats[calaveritaClass][trait_type]) {
      if (stats.ClassStats[calaveritaClass][trait_type][value]) {
        const oldValue = stats.ClassStats[calaveritaClass][trait_type][value];
        stats.ClassStats[calaveritaClass][trait_type][value] = oldValue + 1;
      } else {
        stats.ClassStats[calaveritaClass][trait_type][value] = 1;
      }
    } else {
      stats.ClassStats[calaveritaClass][trait_type] = {
        ...stats.ClassStats[calaveritaClass][trait_type],
        [value]: 1
      };
    }
  });
});

fs.writeFileSync("stats.json", JSON.stringify(stats));
