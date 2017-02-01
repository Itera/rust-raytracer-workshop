mod step_1 {
    use hamcrest::prelude::*;
    use prelude::*;

    #[test]
    fn a_vec3_should_be_added_with_vec3() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(0.0, 1.0, 2.0);

        let c = a + b;

        assert_that!(c.x, is(equal_to(0.0)));
        assert_that!(c.y, is(equal_to(2.0)));
        assert_that!(c.z, is(equal_to(4.0)));
    }

    #[test]
    fn b_vec3_should_be_multiplied_with_vec3() {
        let a = Vec3::new(0.0, 1.0, 2.0);

        let c = 2.0 * a;

        assert_that!(c.x, is(equal_to(0.0)));
        assert_that!(c.y, is(equal_to(2.0)));
        assert_that!(c.z, is(equal_to(4.0)));
    }

    #[test]
    fn c_vec3_should_be_multiplied_with_f64() {
        let a = Vec3::new(0.0, 1.0, 2.0);

        let c = a * 2.0;

        assert_that!(c.x, is(equal_to(0.0)));
        assert_that!(c.y, is(equal_to(2.0)));
        assert_that!(c.z, is(equal_to(4.0)));
    }

}

mod step_2 {
    use hamcrest::prelude::*;
    use prelude::*;

    #[test]
    fn trace_ray_in_scene_should_return_gradient() {
        let scene = Scene::new(vec![]);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));

        let color = ::trace_ray_in_scene(&ray, &scene, 0);

        assert_that!(color, is(equal_to(::gradient(&ray))));
    }
}

mod step_3 {
    use hamcrest::prelude::*;
    use prelude::*;

    #[test]
    fn a_ray_should_calculate_point_along_its_direction() {
        let origin = Vec3::new(1.0, -2.0, 0.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);

        assert_that!(ray.point_along_direction(4.0),
                     is(equal_to(Vec3::new(1.0, 2.0, 0.0))));
    }

    #[test]
    fn b_ray_should_intersect_with_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 1.0, Color::white());
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));

        let i = sphere.intersects(&ray, 0.0, 1000.0).unwrap();

        assert_that!(i.distance, is(equal_to(1.0)));
        assert_that!(i.intersection_point, is(equal_to(Vec3::new(0.0, 0.0, 0.0))));
        assert_that!(i.normal, is(equal_to(Vec3::new(0.0, 0.0, 1.0))));
    }

    #[test]
    fn c_trace_ray_in_scene_should_return_the_color_of_the_intersected_scene() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 1.0, Color::black());
        let scene = Scene::new(vec![Box::new(sphere)]);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));

        let color = ::trace_ray_in_scene(&ray, &scene, 0);

        assert_that!(color, is(equal_to(Color::black())));
    }

    #[test]
    fn c_trace_ray_in_scene_should_return_gradient_if_ray_does_not_intersect() {
        let scene = Scene::new(vec![]);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));

        let color = ::trace_ray_in_scene(&ray, &scene, 0);

        assert_that!(color, is(equal_to(::gradient(&ray))));
    }
}
