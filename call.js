const stravaData = items[0].json;

if (stravaData.length === 0) {
  return [{ json: { message: "No activity was found" } }];
}

let formattedActivities = stravaData.map(activity => {
  return {
    name: activity.name,
    distance: activity.distance,
    moving_time: activity.moving_time,
    type: activity.type,
    start_date: activity.start_date
  };
});

return formattedActivities.map(activity => ({ json: activity }));