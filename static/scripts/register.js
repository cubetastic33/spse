$('.formInput input:not([type=checkbox]), .formInput select, .formInput textarea').each(function() {
	if ($(this).val() !== '') {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	}
	$(this).focusin(() => {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	});
	$(this).focusout(function() {
		if ($(this).val() === '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '1rem',
				'top': '1rem',
				'padding': 0
			}, 80);
		}
	});
	$(this).keyup(function () {
		if (this.id === 'password' && $(this).val().length > 0 && $(this).val().length < 8) {
			$('#password').parent().attr('class', 'formInput error');
			$('#password ~ .helper-text').text('This has to be at least 8 characters');
		} else if (this.id === 'projectAbstract' && $(this).val().split(' ').length > 120) {
			$(this).parent().attr('class', 'formInput error');
			$('#projectAbstract ~ .helper-text').text('The abstract must be lesser than 120 words long');
		} else {
			$(this).parent().attr('class', 'formInput');
			$(`#${this.id} ~ .helper-text`).empty();
		}
	});
});

configureForm();

$('#scienceCategory, #secondaryCategory').change(configureForm);

function configureForm() {
	// Manage the category menus
	if ($('#division').val() === 'science') {
		let subcategoryMenusToHide = [
			'subcategoriesOne',
			'subcategoriesTwo',
			'subcategoriesThree',
			'subcategoriesFour',
			'subcategoriesFive',
			'subcategoriesSix',
			'subcategoriesSeven'
		];
		$(`#${subcategoryMenusToHide.splice(document.getElementById('scienceCategory').selectedIndex, 1)}`)
			.parent()
			.show();
		subcategoryMenusToHide.forEach(id => $(`#${id}`).parent().hide());
		let secondarySubcategoryMenusToHide = [
			'secondarySubcategoriesZero',
			'secondarySubcategoriesOne',
			'secondarySubcategoriesTwo',
			'secondarySubcategoriesThree',
			'secondarySubcategoriesFour',
			'secondarySubcategoriesFive',
			'secondarySubcategoriesSix',
			'secondarySubcategoriesSeven'
		];
		$(`#${secondarySubcategoryMenusToHide.splice(document.getElementById('secondaryCategory').selectedIndex, 1)}`)
			.parent()
			.show();
		secondarySubcategoryMenusToHide.forEach(id => $(`#${id}`).parent().hide());
	} else {
		$('.scienceField').hide();
	}
}

$('#stepOne button').click(() => {
	if ($('#name').val().length === 0) {
		$('#name').parent().attr('class', 'formInput error');
		$('#name ~ .helper-text').text('Enter your name');
		showToast('Errors in form');
		return;
	}
	if ($('#email').val().length === 0 || $('#email').is(':invalid')) {
		$('#email').parent().attr('class', 'formInput error');
		$('#email ~ .helper-text').text('Enter a valid email address');
		showToast('Errors in form');
		return;
	}
	if ($('#phone').is(':invalid')) {
		$('#phone').parent().attr('class', 'formInput error');
		$('#phone ~ .helper-text').text('Enter a valid phone number or none at all');
		showToast('Errors in form');
		return;
	}
	if ($('#password').val().length < 8) {
		$('#password').parent().attr('class', 'formInput error');
		$('#password ~ .helper-text').text('This has to be at least 8 characters');
		showToast('Errors in form');
		return;
	}

	$.ajax({
		type: 'POST',
		url: '/email_available',
		data: {
			email: $('#email').val()
		},
		success: result => {
			console.log(result);
			if (result === 'true') {
				$('#stepOne').hide();
				$('#stepTwo').show();
			} else {
				$('#email').parent().attr('class', 'formInput error');
				$('#email ~ .helper-text').text('This email address is already used');
			}
		}
	});
});

$('#stepTwo .unelevatedButton').click(() => {
	if ($('#section').val().length === 0) {
		$('#section').parent().attr('class', 'formInput error');
		$('#section ~ .helper-text').text('Enter your section');
		showToast('Errors in form');
		return;
	}
	$('#projectAbstract').val($('#division').val() === 'science' ?
	'This must be a short description of your project. You can try to enumerate the key features of your project, or the problems it tries to solve. You can link to attachments if you want. Please note that you can easily edit the project title and abstract later.' :
	'(Optional) You can give a short description of your project here.');
	$(`#projectAbstract + label`).animate({
		'fontSize': '0.8rem',
		'top': '-0.7rem',
		'padding': '0.25rem'
	}, 80);
	$('#stepThree').attr('class', $('#division').val());
	configureForm();
	$('#stepTwo').hide();
	$('#stepThree').show();
});

$('#stepTwo .textButton').click(() => {
	$('#stepTwo').hide();
	$('#stepOne').show();
});

$('#stepThree .textButton').click(() => {
	$('#stepThree').hide();
	$('#stepTwo').show();
});

$('#categoriesButton').click(() => {
	$('.overlay').show();
	$('.dialog').show('slow');
});

$('.overlay').click(() => {
	$('.dialog').hide('slow', () => $('.overlay').hide());
});

$('form').submit(e => e.preventDefault());

$('#submit').click(() => {
	if ($('#projectTitle').val().length === 0) {
		$('#projectTitle').parent().attr('class', 'formInput error');
		$('#projectTitle ~ .helper-text').text('Enter a project title');
		showToast('Errors in form');
		return;
	}
	if ($('#division').val() === 'science' && $('#projectAbstract').val().split(' ').length < 50) {
		$('#projectAbstract').parent().attr('class', 'formInput error');
		$('#projectAbstract ~ .helper-text').text('The project abstract must be at least 50 words long');
		showToast('Errors in form');
		return;
	}
	if ($('#projectAbstract').val().split(' ').length > 120) {
		$('#projectAbstract').parent().attr('class', 'formInput error');
		$('#projectAbstract ~ .helper-text').text('The abstract must be lesser than 120 words long');
		showToast('Errors in form');
		return;
	}
	if ($('#division').val() === 'science' && !$('#guidelinesConsent').prop('checked')) {
		showToast("You can't participate if you haven't read the guidelines");
		return;
	}
	if ($('#division').val() === 'science' && !$('#rulesConsent').prop('checked')) {
		showToast("You can't participate if you don't agree with the rules. If you still want to try, contact the administrators.");
		return;
	}
	if ($('#section').val().length === 0) {
		$('#section').parent().attr('class', 'formInput error');
		$('#section ~ .helper-text').text('Enter your section');
		showToast('Enter your section');
		return;
	}
	if ($('#name').val().length === 0) {
		$('#name').parent().attr('class', 'formInput error');
		$('#name ~ .helper-text').text('Enter your name');
		showToast('Enter your name');
		return;
	}
	if ($('#email').val().length === 0 || $('#email').is(':invalid')) {
		$('#email').parent().attr('class', 'formInput error');
		$('#email ~ .helper-text').text('Enter a valid email address');
		showToast('Enter a valid email address');
		return;
	}
	if ($('#phone').is(':invalid')) {
		$('#phone').parent().attr('class', 'formInput error');
		$('#phone ~ .helper-text').text('Enter a valid phone number or none at all');
		showToast('Enter a valid phone number or none at all');
		return;
	}
	if ($('#password').val().length < 8) {
		$('#password').parent().attr('class', 'formInput error');
		$('#password ~ .helper-text').text('This has to be at least 8 characters');
		showToast('The password has to be at least 8 characters');
		return;
	}
	showToast('Please wait...', 10000);
	let subcategoryMenus = [
		'subcategoriesOne',
		'subcategoriesTwo',
		'subcategoriesThree',
		'subcategoriesFour',
		'subcategoriesFive',
		'subcategoriesSix',
		'subcategoriesSeven'
	];
	let secondarySubcategoryMenus = [
		'secondarySubcategoriesZero',
		'secondarySubcategoriesOne',
		'secondarySubcategoriesTwo',
		'secondarySubcategoriesThree',
		'secondarySubcategoriesFour',
		'secondarySubcategoriesFive',
		'secondarySubcategoriesSix',
		'secondarySubcategoriesSeven'
	];
	let division = $('#division').val();
	let category = $('#scienceCategory').val();
	if (division === 'socialstudies') {
		category = $('#socialstudiesCategory').val();
	} else if (division === 'languages') {
		category = $('#languagesCategory').val();
	}
	$.ajax({
		type: 'POST',
		url: '/register',
		data: {
			name: $('#name').val(),
			email: $('#email').val(),
			phone: $('#phone').val().length === 0 ? undefined : $('#phone').val(),
			password: $('#password').val(),
			grade: $('#class').val(),
			section: $('#section').val(),
			division: division,
			category: ['science', 'socialstudies', 'languages'].includes(division) ? category : undefined,
			subcategory: division === 'science' ? $(`#${subcategoryMenus[document.getElementById('scienceCategory').selectedIndex]}`).val() : undefined,
			secondary_category: $('#secondaryCategory').val() === 'None' || division !== 'science' ? undefined : $('#secondaryCategory').val(),
			secondary_subcategory: $('#secondaryCategory').val() === 'None' || division !== 'science' ? undefined :
				$(`#${secondarySubcategoryMenus[document.getElementById('secondaryCategory').selectedIndex]}`).val(),
			project_title: $('#projectTitle').val(),
			project_abstract: $('#projectAbstract').val().length === 0 ? undefined : $('#projectAbstract').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.href = '/project';
		}
	});
});
